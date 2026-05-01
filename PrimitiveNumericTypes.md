# Primitive Numeric Types in Rust
## 1. Signed Integers
### These can store both negative and positive values. The "i" stands for integer.

- i8: 8-bit (Range: -128 to 127)
- i16: 16-bit (Range: -32,768 to 32,767)
- i32: 32-bit (Range: -2,147,483,648 to 2,147,483,647) — The Rust default.
- i64: 64-bit (Range: approx. -9 quintillion to 9 quintillion)
- i128: 128-bit
- isize: Architecture-dependent (32 or 64-bit). Primarily used for indexing collections.

## 2. Unsigned Integers
### These store only positive values (zero and up). The "u" stands for unsigned.

- u8: 8-bit (Range: 0 to 255)
- u16: 16-bit (Range: 0 to 65,535)
- u32: 32-bit (Range: 0 to 4,294,967,295)
- u64: 64-bit
- u128: 128-bit
- usize: Architecture-dependent. Widely used for [vector and array](https://doc.rust-lang.org/std/primitive.usize.html) sizes.

## 3. Floating-Point Types
### Used for numbers with decimal points.

- f32: Single-precision (32-bit).
- f64: Double-precision (64-bit) — The Rust default, as it provides higher precision on modern CPUs with minimal performance impact.

