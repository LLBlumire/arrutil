// This package contains a number of utility functions, which allow for the easier manipulation of
// arrays, with functionality such as converting to them from slices.

/// Turns a slice into a reference to an array
///
/// Returns `None` if `N` is lower than input slice length
pub fn slice_to_array<T, const N: usize>(source: &[T]) -> Option<&[T; N]> {
    if source.len() < N {
        None
    } else {
        Some(unsafe { slice_to_array_unchecked(source) })
    }
}

/// Turns a mutable slice into a mutable reference to an array
///
/// Returns `None` if `N` is lower than the input slice length
pub fn slice_to_array_mut<T, const N: usize>(source: &mut [T]) -> Option<&mut [T; N]> {
    if source.len() < N {
        None
    } else {
        Some(unsafe { slice_to_array_mut_unchecked(source) })
    }
}

/// Turns a slice into a reference to an array without bounds checking.
pub unsafe fn slice_to_array_unchecked<T, const N: usize>(source: &[T]) -> &[T; N] {
    &*(source.as_ptr() as *const [T; N])
}

/// Turn a mutable slice into a mutable reference to an array without bounds checking.
pub unsafe fn slice_to_array_mut_unchecked<T, const N: usize>(source: &mut [T]) -> &mut [T; N] {
    &mut *(source.as_mut_ptr().cast::<[T; N]>())
}

/// Turns a slice into a reference to an array and a slice starting from the end of the array
///
/// Returns `None` if `N` is shorter than the input slice length
pub fn split_to_array<T, const N: usize>(source: &[T]) -> Option<(&[T; N], &[T])> {
    if source.len() < N {
        None
    } else {
        let (source, tail) = source.split_at(N);
        Some((unsafe { slice_to_array_unchecked(source) }, tail))
    }
}

/// Turns a mutable slice into a mutable reference to an array and a mutable slice starting from the end of the array
///
/// Returns `None` if `N` is shorter than the input slice length
pub fn split_to_array_mut<T, const N: usize>(source: &mut [T]) -> Option<(&mut [T; N], &mut [T])> {
    if source.len() < N {
        None
    } else {
        let (source, tail) = source.split_at_mut(N);
        Some((unsafe { slice_to_array_mut_unchecked(source) }, tail))
    }
}

/// Turn a slice into a reference to an array and mutate the original slice to the end of the array
///
/// Returns `None` if `N` is shorter than the input slice length
pub fn split_to_array_scan<'a, T, const N: usize>(source: &mut &'a [T]) -> Option<&'a [T; N]> {
    split_to_array(source).map(|(head, tail)| {
        *source = tail;
        head
    })
}

#[test]
fn slice_to_array_test() {
    let source = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    assert_eq!(
        slice_to_array(&source[..]),
        Some(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
    );
    assert_eq!(slice_to_array(&source[..]), Some(&[1, 2, 3, 4, 5]));
    assert_eq!(slice_to_array(&source[..5]), Some(&[1, 2, 3, 4, 5]));
    assert_eq!(slice_to_array::<_, 6>(&source[..5]), None);

    let source = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    assert_eq!(
        slice_to_array(&source[..]),
        Some(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
    );
    assert_eq!(slice_to_array(&source[..]), Some(&[1, 2, 3, 4, 5]));
    assert_eq!(slice_to_array(&source[..5]), Some(&[1, 2, 3, 4, 5]));
    assert_eq!(slice_to_array::<_, 6>(&source[..5]), None);
}

#[test]
fn slice_to_array_mut_test() {
    let mut source = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    {
        if let Some(arr) = slice_to_array_mut::<_, 4>(&mut source[3..7]) {
            arr[3] = 100;
        }
    }
    assert_eq!(source, [1, 2, 3, 4, 5, 6, 100, 8, 9, 10]);
}

#[test]
fn split_to_array_test() {
    let source = [1, 2, 3, 4, 5];
    assert_eq!(
        split_to_array(&source[..]),
        Some((&[1, 2, 3], &source[3..]))
    );
    assert_eq!(split_to_array::<_, 6>(&source[..]), None);
}

#[test]
fn split_to_array_mut_test() {
    let mut source = [1, 2, 3, 4, 5];
    {
        if let Some((arr, end)) = split_to_array_mut::<_, 3>(&mut source) {
            arr[1] = 100;
            end[1] = 200;
        }
    }
    assert_eq!(source, [1, 100, 3, 4, 200]);
}

#[test]
fn split_to_array_scan_test() {
    let source = [1, 2, 3, 4, 5];
    {
        let ref mut source_ref = &source[..];
        let double: &[u8; 2] = split_to_array_scan(source_ref).unwrap();
        let single: &[u8; 1] = split_to_array_scan(source_ref).unwrap();
        let dual: &[u8; 2] = split_to_array_scan(source_ref).unwrap();

        assert_eq!(double, &[1, 2]);
        assert_eq!(single, &[3]);
        assert_eq!(dual, &[4, 5]);
    }
}
