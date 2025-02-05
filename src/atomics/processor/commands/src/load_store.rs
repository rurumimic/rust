use std::sync::atomic::{AtomicI32, Ordering};

/// ```bash
/// cargo asm --target x86_64-unknown-linux-gnu  --lib load_store::store_x
/// ```
///
/// ```asm
/// commands::load_store::store_x
///     and dword ptr [rdi], 0
///     ret
/// ```
///
/// ```bash
/// cargo asm --target aarch64-unknown-linux-gnu --lib load_store::store_x
/// ```
///
/// ```asm
/// commands::load_store::store_x
///     str wzr, [x0]
///     ret
/// ```
#[inline(never)]
pub fn store_x(x: &mut i32) {
    *x = 0;
}

/// ```bash
/// cargo asm --target x86_64-unknown-linux-gnu  --lib load_store::load_x
/// ```
///
/// ```asm
/// commands::load_store::load_x
///     mov eax, dword ptr [rdi]
///     ret
/// ```
///
/// ```bash
/// cargo asm --target aarch64-unknown-linux-gnu --lib load_store::load_x
/// ```
///
/// ```asm
/// commands::load_store::load_x
///     ldr w0, [x0]
///     ret
/// ```
#[inline(never)]
pub fn load_x(x: &i32) -> i32 {
    *x
}

/// ```bash
/// cargo asm --target x86_64-unknown-linux-gnu  --lib load_store::atomic_store_x
/// ```
///
/// ```asm
/// commands::load_store::atomic_store_x:
///     mov dword ptr [rdi], 0
///     ret
/// ```
///
/// ```bash
/// cargo asm --target aarch64-unknown-linux-gnu --lib load_store::atomic_store_x
/// ```
///
/// ```asm
/// commands::load_store::atomic_store_x:
///     str wzr, [x0]
///     ret
/// ```
#[inline(never)]
pub fn atomic_store_x(x: &AtomicI32) {
    x.store(0, Ordering::Relaxed);
}

/// ```bash
/// cargo asm --target x86_64-unknown-linux-gnu  --lib load_store::atomic_load_x
/// ```
///
/// ```asm
/// commands::load_store::atomic_load_x
///     mov eax, dword ptr [rdi]
///     ret
/// ```
///
/// ```bash
/// cargo asm --target aarch64-unknown-linux-gnu --lib load_store::atomic_load_x
/// ```
///
/// ```asm
/// commands::load_store::atomic_load_x
///     ldr w0, [x0]
///     ret
/// ```
#[inline(never)]
pub fn atomic_load_x(x: &AtomicI32) -> i32 {
    x.load(Ordering::Relaxed)
}
