# [Rustonomicon](https://doc.rust-lang.org/stable/nomicon/)

## Unsafe Rust

### Different in Unsafe Rust

- Dereference raw pointers
- Call unsafe functions (including C functions, compiler intrinsics, and the raw allocator)
- Implement unsafe traits
- Mutate statics
- Access fields of unions

## Content

1. Safe and Unsafe
2. Data Layout: repr(Rust), Exotically Sized Types, Other reprs
3. Ownership: References, Aliasing, Lifetimes, Limits of Lifetimes, Lifetime Elision, Unbounded Lifetimes, Higher-Rank Trait Bounds, Subtyping and Variance, Drop Check, PhantomData, Splitting Borrows
4. Type Conversions: Coercions, The Dot Operator, Casts, Transmutes
5. Uninitialized Memory: Checked, Drop Flags, Unchecked
6. Ownership Based Resource Management: Constructors, Destructors, Leaking
7. Unwinding: Exception Safety, Poisoning
8. Concurrency: Races, Send and Sync, Atomics
9. Implementing Vec: Layout, Allocating, Push and Pop, Deallocating, Deref, Insert and Remove, IntoIter, RawVec, Drain, Handling Zero-Sized Types, Final Code
10. Implementing Arc and Mutex: Arc, Layout, Base Code, Cloning, Dropping, Final Code
11. FFI
12. Beneath std

