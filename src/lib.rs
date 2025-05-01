#![no_std]

// See https://linebender.org/blog/doc-include for this README inclusion strategy
//! [SubsliceToArray]: crate::SubsliceToArray
//! [SubsliceToArrayRef]: crate::SubsliceToArrayRef
//! [SubsliceToArrayMut]: crate::SubsliceToArrayMut
// File links are not supported by rustdoc
//! [LICENSE-APACHE]: https://github.com/robofinch/subslice_to_array/blob/main/LICENSE-APACHE
//! [LICENSE-MIT]: https://github.com/robofinch/subslice_to_array/blob/main/LICENSE-MIT
//!
//! <style>
//! .rustdoc-hidden { display: none; }
//! </style>
#![doc =  include_str!("../README.md")]


/// Conversion from a subslice with copyable data to an array, with compile-time checks
/// on the `START..END` range used to index into a source slice.
pub trait SubsliceToArray<T, const N: usize> {
    /// Compile-time checked version of code like `slice[START..END].try_into().unwrap()`
    /// for converting part of a slice into an array of length `N`.
    ///
    /// This is only implemented for slices `&[T]` where `T` is `Copy`.
    ///
    /// The function confirms at compile time that `START <= END` and `N == END - START`.
    /// A compile-time error is thrown if this requirement is not met.
    ///
    /// This internally uses [`subslice_to_array`]; if you need to explicitly set the `T` or `N`
    /// generics, you should directly use [`subslice_to_array`], but in most cases this wrapper is
    /// more convenient.
    ///
    /// # Panics
    /// Panics if any index in the `START..END` range is out-of-bounds for the provided slice.
    /// We cannot check that at compile time.
    ///
    /// # Examples
    /// ```
    /// use subslice_to_array::SubsliceToArray as _;
    /// let data: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8];
    /// assert_eq!(
    ///     data.subslice_to_array::<0, 4>(),
    ///     [0, 1, 2, 3],
    /// );
    /// assert_eq!(
    ///     data.subslice_to_array::<4, 9>(),
    ///     [4, 5, 6, 7, 8],
    /// );
    ///
    /// fn fn_that_only_gets_a_slice(bytes: &[u8]) -> Option<[u8; 4]> {
    ///     if bytes.len() < 5 {
    ///         None
    ///     } else {
    ///         Some(bytes.subslice_to_array::<1, 5>())
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     fn_that_only_gets_a_slice(data),
    ///     Some([1, 2, 3, 4]),
    /// );
    ///
    /// let data_vec: Vec<u8> = vec![4, 2];
    /// let data_arr: [u8; 2] = data_vec.subslice_to_array::<0, 2>();
    /// assert_eq!(data_arr, [4, 2]);
    ///
    /// // This is a pretty absurd edge case, but it works.
    /// let unit_arr: [(); usize::MAX] = [(); usize::MAX];
    /// let unit_slice: &[()] = unit_arr.as_slice();
    /// let round_trip: [(); usize::MAX] = unit_slice.subslice_to_array::<0, {usize::MAX}>();
    /// ```
    ///
    /// # Compile fail examples
    /// If `END - START` were computed in release mode without checking that `START <= END`,
    /// the below computation would wrap around to `2`. Since we do perform that check, this
    /// fails to compile.
    /// ```compile_fail
    /// # // Should emit E0080, but we can't specify that on stable, only nightly.
    /// use subslice_to_array::SubsliceToArray as _;
    /// let data: &[u32] = &[0];
    /// let data_2: [u32; 2] = data.subslice_to_array::<{usize::MAX}, 1>();
    /// ```
    ///
    /// Below, `END - START` is not equal to `N`.
    /// ```compile_fail
    /// # // Should emit E0080, but we can't specify that on stable, only nightly.
    /// use subslice_to_array::SubsliceToArray as _;
    /// let data: &[u32] = &[0, 1, 2];
    /// let data_3: [u32; 3] = data.subslice_to_array::<1, 3>();
    /// ```
    fn subslice_to_array<const START: usize, const END: usize>(&self) -> [T; N];
}

impl<T: Copy, const N: usize> SubsliceToArray<T, N> for [T] {
    #[inline]
    fn subslice_to_array<const START: usize, const END: usize>(&self) -> [T; N] {
        subslice_to_array::<START, END, T, N>(self)
    }
}

/// Conversion from a subslice to a reference to an array, with compile-time checks
/// on the `START..END` range used to index into a source slice.
pub trait SubsliceToArrayRef<T, const N: usize> {
    /// Compile-time checked version of code like `slice[START..END].try_into().unwrap()`
    /// for converting part of a slice into a reference to an array of length `N`.
    ///
    /// The function confirms at compile time that `START <= END` and `N == END - START`.
    /// A compile-time error is thrown if this requirement is not met.
    ///
    /// This internally uses [`subslice_to_array_ref`]; if you need to explicitly set the
    /// `T` or `N` generics, you should directly use [`subslice_to_array_ref`], but in most cases
    /// this wrapper is more convenient.
    ///
    /// # Panics
    /// Panics if any index in the `START..END` range is out-of-bounds for the provided slice.
    /// We cannot check that at compile time.
    ///
    /// # Examples
    /// ```
    /// use subslice_to_array::SubsliceToArrayRef as _;
    /// let data: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8];
    /// assert_eq!(
    ///     data.subslice_to_array_ref::<0, 4>(),
    ///     &[0, 1, 2, 3],
    /// );
    /// assert_eq!(
    ///     data.subslice_to_array_ref::<4, 9>(),
    ///     &[4, 5, 6, 7, 8],
    /// );
    ///
    /// fn fn_that_only_gets_a_slice(bytes: &[u8]) -> Option<&[u8; 4]> {
    ///     if bytes.len() < 5 {
    ///         None
    ///     } else {
    ///         Some(bytes.subslice_to_array_ref::<1, 5>())
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     fn_that_only_gets_a_slice(data),
    ///     Some(&[1, 2, 3, 4]),
    /// );
    ///
    /// let data_vec: Vec<u8> = vec![4, 2];
    /// let data_arr: &[u8; 2] = data_vec.subslice_to_array_ref::<0, 2>();
    /// assert_eq!(data_arr, &[4, 2]);
    ///
    /// // This is a pretty absurd edge case, but it works.
    /// let unit_arr: [(); usize::MAX] = [(); usize::MAX];
    /// let unit_slice: &[()] = unit_arr.as_slice();
    /// let unit_arr_ref: &[(); usize::MAX] = unit_slice.subslice_to_array_ref::<0, {usize::MAX}>();
    /// ```
    ///
    /// # Compile fail examples
    /// If `END - START` were computed in release mode without checking that `START <= END`,
    /// the below computation would wrap around to `2`. Since we do perform that check, this
    /// fails to compile.
    /// ```compile_fail
    /// # // Should emit E0080, but we can't specify that on stable, only nightly.
    /// use subslice_to_array::SubsliceToArrayRef as _;
    /// let data: &[u32] = &[0];
    /// let data_2: &[u32; 2] = data.subslice_to_array_ref::<{usize::MAX}, 1>();
    /// ```
    ///
    /// Below, `END - START` is not equal to `N`.
    /// ```compile_fail
    /// # // Should emit E0080, but we can't specify that on stable, only nightly.
    /// use subslice_to_array::SubsliceToArrayRef as _;
    /// let data: &[u32] = &[0, 1, 2];
    /// let data_3: &[u32; 3] = data.subslice_to_array_ref::<1, 3>();
    /// ```
    fn subslice_to_array_ref<const START: usize, const END: usize>(&self) -> &[T; N];
}

impl<T, const N: usize> SubsliceToArrayRef<T, N> for [T] {
    #[inline]
    fn subslice_to_array_ref<const START: usize, const END: usize>(&self) -> &[T; N] {
        subslice_to_array_ref::<START, END, T, N>(self)
    }
}

/// Conversion from a subslice to a mutable reference to an array, with compile-time checks
/// on the `START..END` range used to mutably index into a source slice.
pub trait SubsliceToArrayMut<T, const N: usize> {
    /// Compile-time checked version of code like `(&mut slice[START..END]).try_into().unwrap()`
    /// for converting part of a mutable slice into a mutable reference to an array of length `N`.
    ///
    /// The function confirms at compile time that `START <= END` and `N == END - START`.
    /// A compile-time error is thrown if this requirement is not met.
    ///
    /// This internally uses [`subslice_to_array_mut`]; if you need to explicitly set the
    /// `T` or `N` generics, you should directly use [`subslice_to_array_mut`], but in most cases
    /// this wrapper is more convenient.
    ///
    /// # Panics
    /// Panics if any index in the `START..END` range is out-of-bounds for the provided slice.
    /// We cannot check that at compile time.
    ///
    /// # Examples
    /// ```
    /// use subslice_to_array::SubsliceToArrayMut as _;
    /// let data: &mut [u8] = &mut [0, 1, 2, 3, 4];
    /// *data.subslice_to_array_mut::<1, 3>() = 0xffff_u16.to_le_bytes();
    /// assert_eq!(
    ///     data,
    ///     &mut [0, 255, 255, 3, 4],
    /// );
    ///
    /// let data: &mut [u8] = &mut [0, 1, 2, 3, 4, 5, 6, 7, 8];
    /// assert_eq!(
    ///     data.subslice_to_array_mut::<0, 4>(),
    ///     &mut [0, 1, 2, 3],
    /// );
    /// assert_eq!(
    ///     data.subslice_to_array_mut::<4, 9>(),
    ///     &mut [4, 5, 6, 7, 8],
    /// );
    ///
    /// fn fn_that_only_gets_a_slice(bytes: &mut [u8]) -> Option<&mut [u8; 4]> {
    ///     if bytes.len() < 5 {
    ///         None
    ///     } else {
    ///         Some(bytes.subslice_to_array_mut::<1, 5>())
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     fn_that_only_gets_a_slice(data),
    ///     Some(&mut [1, 2, 3, 4]),
    /// );
    ///
    /// let mut data_vec: Vec<u8> = vec![4, 2];
    /// let data_arr: &mut [u8; 2] = data_vec.subslice_to_array_mut::<0, 2>();
    /// assert_eq!(data_arr, &mut [4, 2]);
    ///
    /// // This is a pretty absurd edge case, but it works.
    /// let mut unit_arr: [(); usize::MAX] = [(); usize::MAX];
    /// let unit_slice: &mut [()] = unit_arr.as_mut_slice();
    /// let unit_arr_ref: &mut [(); usize::MAX] = unit_slice
    ///     .subslice_to_array_mut::<0, {usize::MAX}>();
    /// ```
    ///
    /// # Compile fail examples
    /// If `END - START` were computed in release mode without checking that `START <= END`,
    /// the below computation would wrap around to `2`. Since we do perform that check, this
    /// fails to compile.
    /// ```compile_fail
    /// # // Should emit E0080, but we can't specify that on stable, only nightly.
    /// use subslice_to_array::SubsliceToArrayMut as _;
    /// let data: &[u32] = &[0];
    /// let data_2: &[u32; 2] = data.subslice_to_array_mut::<{usize::MAX}, 1>();
    /// ```
    ///
    /// Below, `END - START` is not equal to `N`.
    /// ```compile_fail
    /// # // Should emit E0080, but we can't specify that on stable, only nightly.
    /// use subslice_to_array::SubsliceToArrayMut as _;
    /// let data: &[u32] = &[0, 1, 2];
    /// let data_3: &[u32; 3] = data.subslice_to_array_mut::<1, 3>();
    /// ```
    fn subslice_to_array_mut<const START: usize, const END: usize>(&mut self) -> &mut [T; N];
}

impl<T, const N: usize> SubsliceToArrayMut<T, N> for [T] {
    #[inline]
    fn subslice_to_array_mut<const START: usize, const END: usize>(&mut self) -> &mut [T; N] {
        subslice_to_array_mut::<START, END, T, N>(self)
    }
}

/// Version of [`SubsliceToArray::subslice_to_array`] with all generic parameters listed.
/// You should probably use this function (and a turbofish) only if the compiler cannot infer
/// `T` or `N`.
///
/// See [`SubsliceToArray::subslice_to_array`] for a more convenient function with identical
/// behavior, and for documentation about this function's behavior.
#[inline]
pub fn subslice_to_array<const START: usize, const END: usize, T: Copy, const N: usize>(
    slice: &[T],
) -> [T; N] {
    *subslice_to_array_ref::<START, END, T, N>(slice)
}

/// Version of [`SubsliceToArrayRef::subslice_to_array_ref`] with all generic parameters listed.
/// You should probably use this function (and a turbofish) only if the compiler cannot infer
/// `T` or `N`.
///
/// See [`SubsliceToArrayRef::subslice_to_array_ref`] for a more convenient function with identical
/// behavior, and for documentation about this function's behavior.
#[inline]
pub fn subslice_to_array_ref<const START: usize, const END: usize, T, const N: usize>(
    slice: &[T],
) -> &[T; N] {
    const {
        assert!(
            START.checked_add(N).is_some(),
            "`START + N` would overflow a usize in subslice_to_array or subslice_to_array_ref",
        );
        assert!(
            // Note that `N` is nonzero and `START + N` does not overflow,
            // so this both ensures `START <= END` and `N == END - START`.
            START + N == END,
            "subslice_to_array or subslice_to_array_ref was called with incorrect START/END bounds",
        );
    }

    // The slice has the same length as the target array, so `try_into` succeeds
    slice[START..END].try_into().unwrap()
}

/// Version of [`SubsliceToArrayMut::subslice_to_array_mut`] with all generic parameters listed.
/// You should probably use this function (and a turbofish) only if the compiler cannot infer
/// `T` or `N`.
///
/// See [`SubsliceToArrayMut::subslice_to_array_mut`] for a more convenient function with identical
/// behavior, and for documentation about this function's behavior.
#[inline]
pub fn subslice_to_array_mut<const START: usize, const END: usize, T, const N: usize>(
    slice: &mut [T],
) -> &mut [T; N] {
    const {
        assert!(
            START.checked_add(N).is_some(),
            "`START + N` would overflow a usize in subslice_to_array_mut",
        );
        assert!(
            // Note that `N` is nonzero and `START + N` does not overflow,
            // so this both ensures `START <= END` and `N == END - START`.
            START + N == END,
            "subslice_to_array_mut was called with incorrect START/END bounds",
        );
    }

    // The slice has the same length as the target array, so `try_into` succeeds
    (&mut slice[START..END]).try_into().unwrap()
}
