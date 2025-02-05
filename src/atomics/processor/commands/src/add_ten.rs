/// ```bash
/// cargo asm --target x86_64-unknown-linux-gnu  --lib add_ten
/// ```
///
/// ```asm
/// commands::add_ten::add_ten:
///     add dword ptr [rdi], 10
///     ret
/// ```
///
/// ```bash
/// cargo asm --target aarch64-unknown-linux-gnu --lib add_ten
/// ```
///
/// ```asm
/// commands::add_ten::add_ten:
///     ldr w8, [x0]
///     add w8, w8, #10
///     str w8, [x0]
///     ret
/// ```
#[inline(never)]
pub fn add_ten(num: &mut i32) {
    *num += 10;
}
