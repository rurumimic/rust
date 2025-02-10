use std::sync::atomic::{AtomicI32, Ordering};

/// ```bash
/// cargo asm --target x86_64-unknown-linux-gnu  --lib read_modify_write::add_x
/// ```
///
/// ```asm
/// commands::read_modify_write::add_x
///     add dword ptr [rdi], 10
///     ret
/// ```
///
/// ```bash
/// cargo asm --target aarch64-unknown-linux-gnu --lib read_modify_write::add_x
/// ```
///
/// ```asm
/// commands::read_modify_write::add_x
///     ldr w8, [x0]
///     add w8, w8, #10
///     str w8, [x0]
///     ret
/// ```
#[inline(never)]
pub fn add_x(x: &mut i32) {
    *x += 10;
}

/// ```bash
/// cargo asm --target x86_64-unknown-linux-gnu  --lib read_modify_write::atomic_add_x
/// ```
///
/// ```asm
/// commands::read_modify_write::atomic_add_x
///     lock add        dword ptr [rdi], 10
///     ret
/// ```
///
/// ```bash
/// cargo asm --target aarch64-unknown-linux-gnu --lib read_modify_write::atomic_add_x
/// ```
///
/// ```asm
/// commands::read_modify_write::atomic_add_x
///     str x30, [sp, #-16]!
///     mov x1, x0
///     mov w0, #10
///     bl __aarch64_ldadd4_relax
///     ldr x30, [sp], #16
///     ret
/// ```
#[inline(never)]
pub fn atomic_add_x(x: &AtomicI32) {
    x.fetch_add(10, Ordering::Relaxed);
}

/// ```bash
/// cargo asm --target x86_64-unknown-linux-gnu  --lib read_modify_write::atomic_add_return_x
/// ```
///
/// ```asm
/// commands::read_modify_write::atomic_add_return_x
///     push 10
///     pop rax
///     lock xadd       dword ptr [rdi], eax
///     ret
/// ```
///
/// ```bash
/// cargo asm --target aarch64-unknown-linux-gnu --lib read_modify_write::atomic_add_return_x
/// ```
///
/// ```asm
/// commands::read_modify_write::atomic_add_return_x
///     str x30, [sp, #-16]!
///     mov x1, x0
///     mov w0, #10
///     bl __aarch64_ldadd4_relax
///     ldr x30, [sp], #16
///     ret
/// ```
#[inline(never)]
pub fn atomic_add_return_x(x: &AtomicI32) -> i32 {
    x.fetch_add(10, Ordering::Relaxed)
}

/// ```bash
/// cargo asm --target x86_64-unknown-linux-gnu  --lib read_modify_write::atomic_or_return_x
/// ```
///
/// ```asm
/// commands::read_modify_write::atomic_or_return_x
///     mov eax, dword ptr [rdi]
/// .LBB7_1:
///     mov ecx, eax
///     or ecx, 10
///     lock cmpxchg    dword ptr [rdi], ecx
///     jne .LBB7_1
///     ret
/// ```
///
/// ```bash
/// cargo asm --target aarch64-unknown-linux-gnu --lib read_modify_write::atomic_or_return_x
/// ```
///
/// ```asm
/// commands::read_modify_write::atomic_or_return_x
///     str x30, [sp, #-16]!
///     mov x1, x0
///     mov w0, #10
///     bl __aarch64_ldset4_relax
///     ldr x30, [sp], #16
///     ret
/// ```
#[inline(never)]
pub fn atomic_or_return_x(x: &AtomicI32) -> i32 {
    x.fetch_or(10, Ordering::Relaxed)
}

/// ```bash
/// cargo asm --target x86_64-unknown-linux-gnu  --lib read_modify_write::compare_exchange_or_return_x
/// ```
///
/// ```asm
/// commands::read_modify_write::compare_exchange_or_return_x
///     mov eax, dword ptr [rdi]
/// .LBB7_1:
///     mov ecx, eax
///     or ecx, 10
///     lock cmpxchg    dword ptr [rdi], ecx
///     jne .LBB8_1
///     ret
/// ```
///
/// ```bash
/// cargo asm --target aarch64-unknown-linux-gnu --lib read_modify_write::compare_exchange_or_return_x
/// ```
///
/// ```asm
/// commands::read_modify_write::compare_exchange_or_return_x
///     stp x30, x21, [sp, #-32]!
///     stp x20, x19, [sp, #16]
///     ldr w20, [x0]
///     mov x19, x0
///     mov w21, #10
/// .LBB8_1:
///     orr w1, w20, w21
///     mov w0, w20
///     mov x2, x19
///     bl __aarch64_cas4_relax
///     cmp w0, w20
///     mov w20, w0
///     b.ne .LBB8_1
///     mov w0, w20
///     ldp x20, x19, [sp, #16]
///     ldp x30, x21, [sp], #32
///     ret
/// ```
#[inline(never)]
pub fn compare_exchange_or_return_x(x: &AtomicI32) -> i32 {
    let mut current = x.load(Ordering::Relaxed);
    loop {
        let new = current | 10;
        match x.compare_exchange(current, new, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(v) => return v,
            Err(v) => current = v,
        }
    }
}

/// ```bash
/// cargo asm --target x86_64-unknown-linux-gnu  --lib read_modify_write::compare_exchange_weak
/// ```
///
/// ```asm
/// commands::read_modify_write::compare_exchange_weak
///     mov ecx, 6
///     mov eax, 5
///     lock cmpxchg    dword ptr [rdi], ecx
///     ret
/// ```
///
/// ```bash
/// cargo asm --target aarch64-unknown-linux-gnu --lib read_modify_write::compare_exchange_weak
/// ```
///
/// ```asm
/// commands::read_modify_write::compare_exchange_weak
///     str x30, [sp, #-16]!
///     mov x2, x0
///     mov w0, #5
///     mov w1, #6
///     bl __aarch64_cas4_relax
///     ldr x30, [sp], #16
///     ret
/// ```
#[inline(never)]
pub fn compare_exchange_weak(x: &AtomicI32) {
    let _ = x.compare_exchange_weak(5, 6, Ordering::Relaxed, Ordering::Relaxed);
}

/// ```bash
/// cargo asm --target x86_64-unknown-linux-gnu  --lib read_modify_write::compare_exchange_x
/// ```
///
/// ```asm
/// commands::read_modify_write::compare_exchange_x
///     mov ecx, 6
///     mov eax, 5
///     lock cmpxchg    dword ptr [rdi], ecx
///     ret
/// ```
///
/// ```bash
/// cargo asm --target aarch64-unknown-linux-gnu --lib read_modify_write::compare_exchange_x
/// ```
///
/// ```asm
/// commands::read_modify_write::compare_exchange_x
///     str x30, [sp, #-16]!
///     mov x2, x0
///     mov w0, #5
///     mov w1, #6
///     bl __aarch64_cas4_relax
///     ldr x30, [sp], #16
///     ret
/// ```
#[inline(never)]
pub fn compare_exchange_x(x: &AtomicI32) {
    let _ = x.compare_exchange(5, 6, Ordering::Relaxed, Ordering::Relaxed);
}
