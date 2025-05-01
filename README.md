<div align="center" class="rustdoc-hidden">
<h1> Subslice to Array </h1>
</div>

Provides functions and traits for easily extracting a subslice from a slice and converting
the subslice to an array, array reference, or mutable array reference, with compile-time checks on
the subslice's length.

Exclusively uses const generics instead of macros.

The three traits [`SubsliceToArray`], [`SubsliceToArrayRef`], and [`SubsliceToArrayMut`]
are the main way this crate should be used; three corresponding functions are provided in case
a full turbofish is needed for specifying the slice's type or the desired array length.

## Motivation

It is already possible to convert a subslice into an array with runtime validity checks
(some of which will likely be optimized away, if the index range is constant):
```rust
let data_slice: &[u32] = &[1, 2, 3];
let first_two: [u32; 2] = data_slice[0..2].try_into().unwrap();
```

However, the following code still compiles, and would panic at runtime:
```rust,should_panic
let data_slice: &[u32] = &[1, 2, 3];
let first_two: [u32; 2] = data_slice[1..2].try_into().unwrap();
```

For the sake of catching bugs in advance, it would be preferable to perform checks at compile time
for cases similar to above, where the range used to create a subslice (from some source slice)
is statically known. Additionally, given that using macros can impact compile times, and given how
simply these checks can be performed with const generics, it would be preferable to use const
generics in stable versions of Rust which support the necessary features.

This crate successfully shifts some of these checks to compile time; it confirms that the range is
valid -- i.e., the start of the range is less than or equal to the end of the range -- and that the
length of the range is equal to the length of the desired array length. This is sufficient to
imply that `.try_into().unwrap()` would not panic. However, we do not and cannot check at compile
time whether the slice is too short; the conversion functions here will panic (at runtime) if
indices in a range are out-of-bounds for a slice.

## Examples
```
use subslice_to_array::{SubsliceToArray as _, SubsliceToArrayMut as _, SubsliceToArrayMut as _};
let data: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8];
assert_eq!(
    data.subslice_to_array::<0, 4>(),
    [0, 1, 2, 3],
);
assert_eq!(
    data.subslice_to_array::<4, 9>(),
    [4, 5, 6, 7, 8],
);

let data: &mut [u8] = &mut [0, 1, 2, 3, 4];
*data.subslice_to_array_mut::<1, 3>() = 0xffff_u16.to_le_bytes();
assert_eq!(
    data,
    &mut [0, 255, 255, 3, 4],
);

fn fn_that_only_gets_a_slice(bytes: &[u8]) -> Option<&[u8; 4]> {
    if bytes.len() < 5 {
        None
    } else {
        Some(bytes.subslice_to_array_ref::<1, 5>())
    }
}

assert_eq!(
    fn_that_only_gets_a_slice(data),
    Some(&[255, 255, 3, 4]),
);
```

## Similar crates
TODO

[`SubsliceToArray`]: https://docs.rs/subslice-to-array/latest/subslice_to_array/trait.SubsliceToArray.html
[`SubsliceToArrayRef`]: https://docs.rs/subslice-to-array/latest/subslice_to_array/trait.SubsliceToArrayRef.html
[`SubsliceToArrayMut`]: https://docs.rs/subslice-to-array/latest/subslice_to_array/trait.SubsliceToArrayMut.html
