#![no_std]

/// Compile-time checked version of code like `slice[START..END].try_into().unwrap()`
/// for converting a slice into an array of length `N`.
///
/// The function confirms at compile time that `START <= END` and `N == END - START`.
/// A compile-time error is thrown if this requirement is not met.
///
/// # Panics
/// Panics if any index in the `START..END` range is out-of-bounds for the provided slice.
/// We cannot check that at compile time.
///
/// # Examples
/// ```
/// # use subslice_to_array::slice_to_array;
/// let data: [u8; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];
/// assert_eq!(
///     slice_to_array::<0, 4, _, 4>(data.as_slice()),
///     [0, 1, 2, 3],
/// );
/// assert_eq!(
///     slice_to_array::<4, 9, _, 5>(data.as_slice()),
///     [4, 5, 6, 7, 8],
/// );
///
/// fn fn_that_only_gets_a_slice(bytes: &[u8]) -> Option<[u8; 4]> {
///     if bytes.len() < 5 {
///         None
///     } else {
///         Some(slice_to_array::<1, 5, _, 4>(bytes))
///     }
/// }
///
/// assert_eq!(
///     fn_that_only_gets_a_slice(data.as_slice()),
///     Some([1, 2, 3, 4]),
/// );
///
/// let data_vec: Vec<u8> = vec![4, 2];
/// let data_arr: [u8; 2] = slice_to_array::<0, 2, _, 2>(&data_vec);
/// assert_eq!(data_arr, [4, 2]);
///
/// // This is a pretty absurd edge case, but it works.
/// let unit_arr: [(); usize::MAX] = [(); usize::MAX];
/// ```
///
/// # Compile fail examples
/// If `END - START` were computed in release mode without checking that `START <= END`,
/// the below computation would wrap around to `2`.
/// ```compile_fail
/// # // Should emit E0080, but we can't specify that on stable, only nightly.
/// # use subslice_to_array::slice_to_array;
/// let data = [0_u32];
/// const START: usize = usize::MAX;
/// const END: usize = 1;
/// let data_2: [u32; 2] = slice_to_array::<START, 1, _, 2>(data.as_slice());
/// ```
///
/// Below, `END - START` is not equal to `N`.
/// ```compile_fail,E0080
/// # // Should emit E0080, but we can't specify that on stable, only nightly.
/// # use subslice_to_array::slice_to_array;
/// let data = [0_u32, 1_u32, 2_u32];
/// let data_3: [u32; 3] = slice_to_array::<1, 3, _, 3>(data.as_slice());
/// ```
///
/// Below, `N` is not equal to the length of `data_3`.
/// ```compile_fail
/// # // Should emit E0308, but we can't specify that on stable, only nightly.
/// # use subslice_to_array::slice_to_array;
/// let data = [0_u32, 1_u32, 2_u32];
/// let data_3: [u32; 3] = slice_to_array::<1, 3, _, 2>(data.as_slice());
/// ```
#[inline]
pub fn slice_to_array<const START: usize, const END: usize, T: Copy, const N: usize>(
    slice: &[T],
) -> [T; N] {
    *slice_to_array_ref::<START, END, T, N>(slice)
}

/// Compile-time checked version of code like `slice[START..END].try_into().unwrap()`
/// for converting a slice into a reference to an array of length `N`.
///
/// The function confirms at compile time that `START <= END` and `N == END - START`.
/// A compile-time error is thrown if this requirement is not met.
///
/// # Panics
/// Panics if any index in the `START..END` range is out-of-bounds for the provided slice.
/// We cannot check that at compile time.
///
/// # Examples
/// ```
/// # use subslice_to_array::slice_to_array_ref;
/// let data: [u8; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];
/// assert_eq!(
///     slice_to_array_ref::<0, 4, _, 4>(data.as_slice()),
///     &[0, 1, 2, 3],
/// );
/// assert_eq!(
///     slice_to_array_ref::<4, 9, _, 5>(data.as_slice()),
///     &[4, 5, 6, 7, 8],
/// );
///
/// fn fn_that_only_gets_a_slice(bytes: &[u8]) -> Option<&[u8; 4]> {
///     if bytes.len() < 5 {
///         None
///     } else {
///         Some(slice_to_array_ref::<1, 5, _, 4>(bytes))
///     }
/// }
///
/// assert_eq!(
///     fn_that_only_gets_a_slice(data.as_slice()),
///     Some(&[1, 2, 3, 4]),
/// );
///
/// let data_vec: Vec<u8> = vec![4, 2];
/// let data_arr: &[u8; 2] = slice_to_array_ref::<0, 2, _, 2>(&data_vec);
/// assert_eq!(data_arr, &[4, 2]);
/// ```
#[inline]
pub fn slice_to_array_ref<const START: usize, const END: usize, T, const N: usize>(
    slice: &[T],
) -> &[T; N] {
    const {
        assert!(
            START.checked_add(N).is_some(),
            "`START + N` would overflow a usize in slice_to_array or slice_to_array_ref",
        );
        assert!(
            // Note that `N` is nonzero and `START + N` does not overflow,
            // so this both ensures `START <= END` and `N == END - START`.
            START + N == END,
            "slice_to_array or slice_to_array_ref was called with incorrect START/END bounds",
        );
    }

    // The slice has the same length as the target array, so `try_into` succeeds
    slice[START..END].try_into().unwrap()
}

/// Compile-time checked version of code like `(&mut slice[START..END]).try_into().unwrap()`
/// for converting a mutable slice reference into a mutable reference to an array of length `N`.
///
/// The function confirms at compile time that `START <= END` and `N == END - START`.
/// A compile-time error is thrown if this requirement is not met.
///
/// # Panics
/// Panics if any index in the `START..END` range is out-of-bounds for the provided slice.
/// We cannot check that at compile time.
///
/// # Examples
/// ```
/// # use subslice_to_array::slice_to_array_mut;
/// let mut data: [u8; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];
/// assert_eq!(
///     slice_to_array_mut::<0, 4, _, 4>(data.as_mut_slice()),
///     &mut [0, 1, 2, 3],
/// );
/// assert_eq!(
///     slice_to_array_mut::<4, 9, _, 5>(data.as_mut_slice()),
///     &mut [4, 5, 6, 7, 8],
/// );
///
/// fn fn_that_only_gets_a_slice(bytes: &mut [u8]) -> Option<&mut [u8; 4]> {
///     if bytes.len() < 5 {
///         None
///     } else {
///         Some(slice_to_array_mut::<1, 5, _, 4>(bytes))
///     }
/// }
///
/// assert_eq!(
///     fn_that_only_gets_a_slice(data.as_mut_slice()),
///     Some(&mut [1, 2, 3, 4]),
/// );
///
/// let mut data_vec: Vec<u8> = vec![4, 2];
/// let data_arr: &mut [u8; 2] = slice_to_array_mut::<0, 2, _, 2>(&mut data_vec);
/// assert_eq!(data_arr, &mut [4, 2]);
/// ```
#[inline]
pub fn slice_to_array_mut<const START: usize, const END: usize, T, const N: usize>(
    slice: &mut [T],
) -> &mut [T; N] {
    const {
        assert!(
            START.checked_add(N).is_some(),
            "`START + N` would overflow a usize in slice_to_array_mut",
        );
        assert!(
            // Note that `N` is nonzero and `START + N` does not overflow,
            // so this both ensures `START <= END` and `N == END - START`.
            START + N == END,
            "slice_to_array_mut was called with incorrect START/END bounds",
        );
    }

    // The slice has the same length as the target array, so `try_into` succeeds
    (&mut slice[START..END]).try_into().unwrap()
}
