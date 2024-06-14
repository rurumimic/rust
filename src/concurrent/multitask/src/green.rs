use nix::sys::mman::{mprotect, ProtFlags};
use rand;
use std::alloc::{alloc, dealloc, Layout};
use std::collections::{HashMap, HashSet, LinkedList};
use std::ffi::c_void;
use std::ptr::{self, NonNull};

static mut CTX_MAIN: Option<Box<Registers>> = None;
static mut UNUSED_STACK: (*mut u8, Layout) = (ptr::null_mut(), Layout::new::<u8>());
static mut CONTEXTS: LinkedList<Box<Context>> = LinkedList::new(); // thread run queue
static mut ID: *mut HashSet<u64> = ptr::null_mut(); // thread ids set
static mut MESSAGES: *mut MappedList<u64> = ptr::null_mut(); // message queue
static mut WAITING: *mut HashMap<u64, Box<Context>> = ptr::null_mut(); // waiting thread queue

#[repr(C)] // C calling convention
struct Registers { // 8 * 8 = 64 bytes
    rbx: u64, // base register
    rbp: u64, // base pointer
    r12: u64, // general-purpose register
    r13: u64,
    r14: u64,
    r15: u64,
    rsp: u64, // stack pointer
    rdx: u64, // data register
}

impl Registers {
    fn new(rsp: u64) -> Self {
        Self {
            rbx: 0,
            rbp: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
            rsp,
            rdx: entry_point as u64,
        }
    }
}

extern "C" {
    fn set_context(ctx: *mut Registers) -> u64;
    fn switch_context(ctx: *const Registers) -> !; // never returns
}

// type: type alias
// Entry: a new type
// fn(): function pointer that takes no arguments and returns nothing
type Entry = fn();

// use nix::unistd::sysconf in production
const PAGE_SIZE: usize = 4 * 1024; // 4KiB

struct MappedList<T> {
    map: HashMap<u64, LinkedList<T>>,
}

impl<T> MappedList<T> {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn clear(&mut self) {
        self.map.clear();
    }
}

struct Context {
    regs: Registers,
    stack: *mut u8,
    stack_layout: Layout,
    entry: Entry, // entrypoint
    id: u64,      // thread id
}

impl Context {
    fn get_regs_mut(&mut self) -> *mut Registers {
        &mut self.regs as *mut Registers
    }

    fn get_regs(&self) -> *const Registers {
        &self.regs as *const Registers
    }

    #[inline(never)] // don't inline this function
    fn new(func: Entry, stack_size: usize, id: u64) -> Self {
        let layout = Layout::from_size_align(stack_size, PAGE_SIZE).unwrap();
        let stack = unsafe { alloc(layout) };

        // protect first page. stack overflow protection
        unsafe {
            let addr = NonNull::new(stack)
                .expect("failed to allocate stack")
                .cast::<c_void>();
            mprotect(addr, PAGE_SIZE, ProtFlags::PROT_NONE).unwrap();
        }

        // init registers. stack pointer points to the end of the stack
        let regs = Registers::new(stack as u64 + stack_size as u64);

        Context {
            regs,
            stack,
            stack_layout: layout,
            entry: func,
            id,
        }
    }
}

fn get_id() -> u64 {
    loop {
        let rnd = rand::random::<u64>();
        unsafe {
            if !(*ID).contains(&rnd) {
                (*ID).insert(rnd);
                return rnd;
            }
        }
    }
}

pub fn spawn(func: Entry, stack_size: usize) -> u64 {
    unsafe {
        let id = get_id();
        CONTEXTS.push_back(Box::new(Context::new(func, stack_size, id)));
        schedule();
        id
    }
}

pub fn schedule() {
    unsafe {
        // if there is only one context, return
        if CONTEXTS.len() == 1 {
            return;
        }

        // move the current context to the back of the queue
        let mut ctx = CONTEXTS.pop_front().unwrap();

        // get pointer of registers
        let regs = ctx.get_regs_mut();
        CONTEXTS.push_back(ctx);

        // save registers in stack
        if set_context(regs) == 0 {
            let next = CONTEXTS.front().unwrap();
            switch_context((**next).get_regs());
        }

        rm_unused_stack(); // when the context is switched, remove the unused stack
    }
}

#[no_mangle] // don't mangle the name
pub extern "C" fn entry_point() {
    unsafe {
        // run the entrypoint
        let ctx = CONTEXTS.front().unwrap();
        ((**ctx).entry)();

        // when the entrypoint returns, remove the context

        let ctx = CONTEXTS.pop_front().unwrap();

        (*ID).remove(&ctx.id);

        UNUSED_STACK = ((*ctx).stack, (*ctx).stack_layout);

        match CONTEXTS.front() {
            Some(next) => switch_context((**next).get_regs()),
            None => {
                // if there is no context, switch to the main context
                if let Some(next) = &CTX_MAIN {
                    switch_context(&**next as *const Registers);
                }
            }
        };
    }

    unreachable!();
}

pub fn spwan_from_main(func: Entry, stack_size: usize) {
    unsafe {
        if let Some(_) = &CTX_MAIN {
            panic!("spawn_from_main is called twice");
        }

        CTX_MAIN = Some(Box::new(Registers::new(0)));
        if let Some(ctx) = &mut CTX_MAIN {
            let mut msgs = MappedList::new();
            MESSAGES = &mut msgs as *mut MappedList<u64>;

            let mut waiting = HashMap::new();
            WAITING = &mut waiting as *mut HashMap<u64, Box<Context>>;

            let mut ids = HashSet::new();
            ID = &mut ids as *mut HashSet<u64>;

            if set_context(&mut **ctx as *mut Registers) == 0 {
                CONTEXTS.push_back(Box::new(Context::new(func, stack_size, get_id())));
                let first = CONTEXTS.front().unwrap();
                switch_context(first.get_regs());
            }

            rm_unused_stack();

            CTX_MAIN = None;
            CONTEXTS.clear();
            MESSAGES = ptr::null_mut();
            WAITING = ptr::null_mut();
            ID = ptr::null_mut();

            msgs.clear();
            waiting.clear();
            ids.clear();
        }
    }
}

unsafe fn rm_unused_stack() {
    if UNUSED_STACK.0 != ptr::null_mut() {
        let addr = NonNull::new(UNUSED_STACK.0)
            .expect("failed to deallocate stack")
            .cast::<c_void>();
        mprotect(
            addr,
            PAGE_SIZE,
            ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
        )
        .unwrap();
        dealloc(UNUSED_STACK.0, UNUSED_STACK.1);
        UNUSED_STACK = (ptr::null_mut(), Layout::new::<u8>());
    }
}
