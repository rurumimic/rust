use std::arch::asm;
use std::cell::UnsafeCell;
use std::ptr::null_mut;

#[repr(C)]
struct Node<T> {
    next: *mut Node<T>,
    data: T,
}

#[repr(C)]
pub struct StackHead<T> {
    head: *mut Node<T>,
}

pub struct Stack<T> {
    data: UnsafeCell<StackHead<T>>,
}

impl<T> StackHead<T> {
    fn new() -> Self {
        StackHead { head: null_mut() }
    }

    pub fn push(&mut self, v: T) {
        let node = Box::new(Node {
            next: null_mut(),
            data: v,
        });

        let ptr = Box::into_raw(node);

        let head = &mut self.head as *mut *mut Node<T> as *mut u8 as usize;
        /*
         * self.head (type: *mut Node<T>)
         *     |
         *     v
         * &mut self.head (type: &mut *mut Node<T>)
         *     |
         *     v
         * as *mut *mut Node<T> (type: *mut *mut Node<T>)
         *     | type to byte pointer
         *     v
         * as *mut u8 (type: *mut u8)
         *     | pointer to integer type for asm! macro
         *     v
         * as usize (type: usize)
         *
         */

        unsafe {
            asm!(
                "1:",
                "ldxr {next}, [{head}]", // next = *head, Load the current head atomically
                "str {next}, [{ptr}]", // *ptr = next, Set the new node's next to current head
                "stlxr w10, {ptr}, [{head}]", // *head = ptr, Attempt to store the new node as head
                "cbnz w10, 1b", // if tmp != 0 then goto 1, If store failed, retry
                next = out(reg) _,
                ptr = in(reg) ptr,
                head = in(reg) head,
                out("w10") _);
        };
    }

    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            let head = &mut self.head as *mut *mut Node<T> as *mut u8 as usize;

            let mut result: usize;

            asm!(
                "1:",
                "ldxr {result}, [{head}]", // result = *head, Load the current head atomically
                "cbnz {result}, 2f", // if result != NULL then goto 2, If head is not NULL, proceed
                // if NULL
                "clrex", // clear exclusive lock
                "b 3f", // goto 3, exit
                // if not NULL
                "2:",
                "ldr {next}, [{result}]", // next = *result, Load the next node
                "stxr w10, {next}, [{head}]", // *head = next, Attempt to store the next node as head
                "cbnz w10, 1b", // if tmp != 0 then goto 1, If store failed, retry
                "3:", // Exit
                next = out(reg) _,
                result = out(reg) result,
                head = in(reg) head,
                out("w10") _);

            if result == 0 {
                None
            } else {
                let ptr = result as *mut u8 as *mut Node<T>;
                let head = Box::from_raw(ptr);
                Some((*head).data)
            }
        }
    }
}

impl<T> Drop for StackHead<T> {
    fn drop(&mut self) {
        let mut node = self.head;

        while node != null_mut() {
            let n = unsafe { Box::from_raw(node) };
            node = n.next;
        }
    }
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            data: UnsafeCell::new(StackHead::new()),
        }
    }

    pub fn get_mut(&self) -> &mut StackHead<T> {
        unsafe { &mut *self.data.get() }
    }
}

unsafe impl<T> Sync for Stack<T> {}
unsafe impl<T> Send for Stack<T> {}
