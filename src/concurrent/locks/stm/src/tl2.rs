use std::cell::UnsafeCell;
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{fence, AtomicU64, Ordering};

/*
* stripe and memory size = 2^n bytes
* if stripe size = 1byte, then shift_size = 0
* if stripe size = 2bytes, then shift_size = 1
* if stripe size = 4bytes, then shift_size = 2
* if stripe size = 8bytes, then shift_size = 3
*
* if shift size = 1, then
* Address >> 1 = stripe number
*   0000 >> 1 = 0
*   0001 >> 1 = 0
*   0010 >> 1 = 1
*   0011 >> 1 = 1
*
* if shift size = 3, then
* Address >> 3 = stripe number
*
* 0 0100 >> 3 = 0
* 0 0111 >> 3 = 0
* 0 1000 >> 3 = 1
* 0 1011 >> 3 = 1
* 1 0000 >> 3 = 2
* 1 0011 >> 3 = 2
*/

/*
* 2^9 = 512 = memory size
* 2^3 = 8 = stripe size
* 2^(9-3) = 2^6 = 64 = stripes
*
* 2^9 2^8 2^7 2^6 2^5 2^4 2^3 | 2^2 2^1 2^0
* ----------------------------|-------------
*   0   0   0   0   0   0   0 |  0   0   0
*   0   0   0   0   0   0   1 |  0   0   0
*
*   ...
*
*   0   1   1   1   1   1   1 |  0   0   0
*   1   0   0   0   0   0   0 |  0   0   0
* ----------------------------|-------------
* 2^6 2^5 2^4 2^3 2^2 2^1 2^0 |
*/
const STRIPE_SIZE: usize = 8; // 8 bytes
const MEM_SIZE: usize = 512; // 512 bytes

// memory init, lock, version management
pub struct Memory {
    mem: Vec<u8>,
    lock_ver: Vec<AtomicU64>, // lock&version
    global_clock: AtomicU64,  // globla version-clock
    shift_size: u32,          // address + shift = stripe
}

impl Memory {
    pub fn new() -> Self {
        let mem = [0].repeat(MEM_SIZE);

        // if stripe size = 8bytes, then shift_size = 3
        // usize 8 = 00001000
        let shift: u32 = STRIPE_SIZE.trailing_zeros(); // 3

        let mut lock_ver = Vec::new();
        let stripes = MEM_SIZE >> shift; // 64 = 2^6 = 2^9 / 2^3
        for _ in 0..stripes {
            lock_ver.push(AtomicU64::new(0));
        }

        Memory {
            mem,
            lock_ver,
            global_clock: AtomicU64::new(0),
            shift_size: shift,
        }
    }

    fn inc_global_clock(&mut self) -> u64 {
        self.global_clock.fetch_add(1, Ordering::AcqRel)
    }

    fn get_addr_ver(&self, addr: usize) -> u64 {
        let idx = addr >> self.shift_size;
        let n = self.lock_ver[idx].load(Ordering::Relaxed);
        n & !(1 << 63) // clear lock bit (MSB) and return version
    }

    fn test_not_modify(&self, addr: usize, rv: u64) -> bool {
        let idx = addr >> self.shift_size;
        let n = self.lock_ver[idx].load(Ordering::Relaxed);

        // MSB is lock bit
        // easy to compare with rv
        // if MSB is 1, then n > rv
        // if MSB is 0, then compare a version
        // if memory version is lower, then true = not modify
        // if memory version is higher, then false = modify
        n <= rv
    }

    fn lock_addr(&mut self, addr: usize) -> bool {
        let idx = addr >> self.shift_size;
        match self.lock_ver[idx].fetch_update(
            Ordering::Relaxed, // write
            Ordering::Relaxed, // read
            |val| {
                let n = val & (1 << 63); // lock bit
                if n == 0 {
                    // try to lock
                    Some(val | (1 << 63)) // set lock bit
                                          // if success, return Ok(old value)
                                          // if other thread has lock, return Err(old value)
                } else {
                    // do nothing and return Err(old value)
                    None
                }
            },
        ) {
            Ok(_) => true, // if lock success, return true
            Err(_) => false,
        }
    }

    fn unlock_addr(&mut self, addr: usize) {
        let idx = addr >> self.shift_size;
        self.lock_ver[idx].fetch_and(!(1 << 63), Ordering::Relaxed);
        // MSB = 0
    }
}

pub struct ReadTrans<'a> {
    read_ver: u64,
    is_abort: bool, // if contention is detected, then abort
    mem: &'a Memory,
}

impl<'a> ReadTrans<'a> {
    fn new(mem: &'a Memory) -> Self {
        ReadTrans {
            read_ver: mem.global_clock.load(Ordering::Acquire),
            is_abort: false,
            mem,
        }
    }

    pub fn load(&mut self, addr: usize) -> Option<[u8; STRIPE_SIZE]> {
        if self.is_abort {
            return None;
        }

        //   2^98 7654 3210
        //   0000 0000 1000 = STRIPE_SIZE = 8
        // - 0000 0000 0001
        // ----------------
        //   0000 0000 0111
        // & 0000 1101 1000 = any addr < 512
        // ================
        //   0000 0000 0000
        assert_eq!(addr & (STRIPE_SIZE - 1), 0);

        // if read mem is modify, then abort
        if !self.mem.test_not_modify(addr, self.read_ver) {
            self.is_abort = true;
            return None;
        }

        fence(Ordering::Acquire);

        //        [0, 0, 0, 0, 0, 0, 0, 0]
        //         ↑  ↑  ↑  ↑  ↑  ↑  ↑  ↑
        // [..., addr, ..., addr + STRIPE_SIZE - 1, ...]
        let mut mem = [0; STRIPE_SIZE];
        for (dst, src) in mem
            .iter_mut()
            .zip(self.mem.mem[addr..addr + STRIPE_SIZE].iter())
        {
            *dst = *src;
        }

        fence(Ordering::SeqCst);

        // if read mem is modify, then abort
        if !self.mem.test_not_modify(addr, self.read_ver) {
            self.is_abort = true;
            return None;
        }

        Some(mem)
    }
}

pub struct WriteTrans<'a> {
    read_ver: u64,
    read_set: HashSet<usize>,
    write_set: HashMap<usize, [u8; STRIPE_SIZE]>,
    locked: Vec<usize>, // locked addr, for unlock write-set
    is_abort: bool,
    mem: &'a mut Memory,
}

impl<'a> Drop for WriteTrans<'a> {
    fn drop(&mut self) {
        for addr in self.locked.iter() {
            self.mem.unlock_addr(*addr);
        }
    }
}

impl<'a> WriteTrans<'a> {
    fn new(mem: &'a mut Memory) -> Self {
        WriteTrans {
            read_ver: mem.global_clock.load(Ordering::Acquire),
            read_set: HashSet::new(),
            write_set: HashMap::new(),
            locked: Vec::new(),
            is_abort: false,
            mem,
        }
    }

    pub fn store(&mut self, addr: usize, val: [u8; STRIPE_SIZE]) {
        assert_eq!(addr & (STRIPE_SIZE - 1), 0);
        self.write_set.insert(addr, val);
    }

    pub fn load(&mut self, addr: usize) -> Option<[u8; STRIPE_SIZE]> {
        if self.is_abort {
            return None;
        }

        assert_eq!(addr & (STRIPE_SIZE - 1), 0);

        // different from ReadTrans
        self.read_set.insert(addr);
        if let Some(m) = self.write_set.get(&addr) {
            return Some(*m);
        }

        if !self.mem.test_not_modify(addr, self.read_ver) {
            self.is_abort = true;
            return None;
        }

        fence(Ordering::Acquire);

        let mut mem = [0; STRIPE_SIZE];
        for (dst, src) in mem
            .iter_mut()
            .zip(self.mem.mem[addr..addr + STRIPE_SIZE].iter())
        {
            *dst = *src;
        }

        fence(Ordering::SeqCst);

        if !self.mem.test_not_modify(addr, self.read_ver) {
            self.is_abort = true;
            return None;
        }

        Some(mem)
    }

    // lock all stripes in write-set
    fn lock_write_set(&mut self) -> bool {
        for (addr, _) in self.write_set.iter() {
            if self.mem.lock_addr(*addr) {
                self.locked.push(*addr);
            } else {
                return false;
            }
        }
        true
    }

    fn validate_read_set(&self) -> bool {
        for addr in self.read_set.iter() {
            if self.write_set.contains_key(addr) {
                let ver = self.mem.get_addr_ver(*addr);
                if ver > self.read_ver {
                    return false;
                }
            } else {
                if !self.mem.test_not_modify(*addr, self.read_ver) {
                    return false;
                }
            }
        }
        true
    }

    fn commit(&mut self, ver: u64) {
        for (addr, val) in self.write_set.iter() {
            let addr = *addr as usize;
            for (dst, src) in self.mem.mem[addr..addr + STRIPE_SIZE].iter_mut().zip(val) {
                *dst = *src;
            }
        }

        fence(Ordering::Release);

        for (addr, _) in self.write_set.iter() {
            let idx = addr >> self.mem.shift_size;
            self.mem.lock_ver[idx].store(ver, Ordering::Relaxed);
        }

        self.locked.clear();
    }
}

pub enum STMResult<T> {
    Ok(T),
    Retry,
    Abort,
}

pub struct STM {
    mem: UnsafeCell<Memory>,
}

unsafe impl Sync for STM {}
unsafe impl Send for STM {}

impl STM {
    pub fn new() -> Self {
        STM {
            mem: UnsafeCell::new(Memory::new()),
        }
    }

    pub fn read_transaction<F, R>(&self, f: F) -> Option<R>
    where
        F: Fn(&mut ReadTrans) -> STMResult<R>,
    {
        loop {
            // 1. read global version-clock
            let mut tr = ReadTrans::new(unsafe { &*self.mem.get() });

            // 2. Run Transaction
            match f(&mut tr) {
                STMResult::Abort => return None,
                STMResult::Retry => {
                    if tr.is_abort {
                        continue;
                    }
                    return None;
                }
                STMResult::Ok(val) => {
                    if tr.is_abort {
                        continue;
                    } else {
                        return Some(val); // commit
                    }
                }
            }
        }
    }

    pub fn write_transaction<F, R>(&self, f: F) -> Option<R>
    where
        F: Fn(&mut WriteTrans) -> STMResult<R>,
    {
        loop {
            // 1. read global version-clock
            let mut tr = WriteTrans::new(unsafe { &mut *self.mem.get() });

            // 2. Run Transaction
            let result;
            match f(&mut tr) {
                STMResult::Abort => return None,
                STMResult::Retry => {
                    if tr.is_abort {
                        continue;
                    }
                    return None;
                }
                STMResult::Ok(val) => {
                    if tr.is_abort {
                        continue;
                    }
                    result = val;
                }
            }

            // 3. lock write-set
            if !tr.lock_write_set() {
                continue;
            }

            // 4. increment global version-clock
            let ver = 1 + tr.mem.inc_global_clock();

            // 5. validate read-set
            if tr.read_ver + 1 != ver && !tr.validate_read_set() {
                continue;
            }

            // 6. commit and release
            tr.commit(ver);

            return Some(result);
        }
    }
}

#[macro_export]
macro_rules! load {
    ($t:ident, $a:expr) => {
        if let Some(v) = ($t).load($a) {
            v
        } else {
            return tl2::STMResult::Retry;
        }
    };
}

#[macro_export]
macro_rules! store {
    ($t:ident, $a:expr, $v:expr) => {
        $t.store($a, $v)
    };
}

