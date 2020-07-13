# Flexible fixed-point numbers

This crate intended to simplify fixed-point calculations especially on FPU-less hardware.
To make it possible it introduces generic fixed-point type with usable and flexible operations on it.

## Overview

The introduced numeric type is generic with three type parameters:

* `Radix` - the __base__ of type, 2 for binary point, 10 for decimal point
* `Digits` - the number of valuable digits which represents the __mantissa__
* `Exponent` - the static __exponent__ value of type

So the value of type can be represented as _`mantissa` × `radix` <sup>`exponent`</sup>_.

```rust
# use ufix::{Fix, bin, dec};
# use typenum::*;
#
// Signed binary fixed with 5 bits mantissa and -3 as exponent.
// [5]*2^-3
type BF1 = Fix<P2, P5, N3>;
// or using type alias
type BF2 = bin::Fix<P5, N3>;

// Signed decimal fixed with 12 bits mantissa and -7 as exponent.
// [12]*10^-7
type DF1 = Fix<P12, U10, N7>;
type DF2 = dec::Fix<P12, N7>;

// Unsigned binary fixed with 5 bits mantissa and 3 as exponent.
// [5]*2^3
type BF1 = Fix<U2, P5, P3>;
// or using type alias
type BF2 = bin::UFix<P5, P3>;
```

The `P*` as the radix type parameter means signed type. To create unsigned types you can use `U*` instead.

Unlike well known and widely used __Qn.m__ representation the exponent is not constrained by mantissa bits. It can be less to represent more precisive small values. Also it can be greater than zero to represent less precision bigger values.

```rust
# use ufix::Fix;
# use typenum::*;
#
// [5]*2^-7
// 0b0.0000000 .. 0b0.0011111
type BF1 = Fix<U5, U2, N7>;

// [5]*2^7
// 0b0000000 .. 0b1111100
type BF2 = Fix<U5, U2, P7>;
```

### Optimization techniques

When you targeted to [FPU](https://en.wikipedia.org/wiki/Floating-point_unit)-less hardware in order to get best possible performance and reduce firmware size you should use only binary fixed point arithmetic because internally it operates with integers, and exponent adjustement operations requires only bitwise shifting. Also you should avoid exceeding platform word size when it is possible without lossing required precision.

By default this crate use 32-bit integers as optimal to use on 32-bit processors. When you targeted to 16-bit or 8-bit processor you should use *word16* or *word8* features respectively.

### Safe usage

Fixed point arithmetic has well known problems with overflowing especially on multiplication. Also it has well known problems with precision loss on division.

The simple way to avoid overflow is using value types of double bit-width in operation with following reducing to original width.

For example, in case of multiplication we can cast 32-bit fixed-point number to 64-bit with same base and exponent. As result we get 64-bit fixed-point number with exponent, which equals a sum of arguments exponents.

In case of division to prevent lossing precision we can cast 32-bit numerator to 64-bit with double exponent and keep 32-bit denominator as is. In result we get 32-bit number with exponent, which equals a difference of numerator (after cast) and denominator exponents.

See examples below:

```rust
use ufix::{Cast, bin::{Fix}};
use typenum::*;

let a = Fix::<P16, N8>::from(123.45);
let b = Fix::<P16, N8>::from(78.9);

// The multiplication without overflow
let c = a * b;

assert_eq!(c, Fix::<P32, N16>::from(9739.95047));
```

```rust
use ufix::{Cast, bin::{Fix}};
use typenum::*;

let a = Fix::<P16, N8>::from(123.45);
let b = Fix::<P16, N8>::from(78.9);

// The division without precision loss
let c = Fix::<P32, N16>::cast(a) / b;

assert_eq!(c, Fix::<P16, N8>::from(1.5647));
```

### Evolution of `Fix` type

0. **fix** crate by _Curtis McEnroe_.

   That crate defines a simple fixed point type which support generic *base* and *exponent*.
   This work is started as successor of **fix**.

1. Changing meaning of `Bits` parameter from *mantisa type* to *number of mantissa bits*.

   In order to get more flexibility of fixed-point type we need know number of valuable bits of mantissa.
   So we would be able select appropriate types of results of operations easy.
   Also this make it possible to operate with operands with different mantissa and exponent.

2. Changing meaning of `Bits` parameter from *number of mantissa bits* to *number of mantissa digits according to base*.

   Actually it is more easy to operate with actual number of digits in number instead of binary bits.
   Because the digits depended from base, so for binary fixed-point types it is still bits as before.