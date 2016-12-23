//! This package contains a number of utility macros, which allow for the easier manipulation of
//! arrays, with functionality such as converting to them from slices. It will gain more methods as
//! time advances. 
//!
//! To get around the restriction of a lack of type level integers in rust, every macro generates a
//! local function (accessible only inside the macro) that handles the unsafe code and manipulation
//! of slices and arrays. This is to ensure that lifetimes behave as expected. Every macro has
//! documentation for it's inner macro function attached to it. The macros themselves have
//! identical syntax to these function with the addition of all sizes for arrays being passed as
//! literals.
//!
//! If there are any macros you think should be in this crate and are missing, raise an issue on
//! this crates repository and if they belong here I'll add them as soon as possible!

/// You can use `slice_to_array` to create a reference to an array from a slice. 
///
/// Internally, it has the signature:
///
/// ```
/// fn slice_to_array<T>(source: &[T]) -> Option<&[T; N]>
/// ```
///
/// Externally, it should be seen as having the signature
///
/// ```
/// slice_to_array!(source: &[T], len: N) -> Option<&[T; N>
/// ```
///
/// Where N is an integer literal and a valid array length.
///
/// Returns `None` when length is longer than slice.
#[macro_export]
macro_rules! slice_to_array {
    ($source:expr, $len:expr) => {{
        #[inline]
        fn slice_to_array<T>(source: &[T]) -> Option<&[T; $len]> {
            if source.len() < $len {
                None
            } else {
                Some(unsafe {
                    slice_to_array_unchecked!(source, $len)
                })
            }
        }
        
        slice_to_array($source)
    }}
}

/// You can use `slice_to_array_mut` to create a reference to an array from a mutable slice. 
///
/// Internally, it has the signature:
///
/// ```
/// fn slice_to_array_mut<T>(source: &mut [T]) -> Option<&mut [T; N]>
/// ```
///
/// Externally, it should be seen as having the signature
///
/// ```
/// slice_to_array_mut!(source: &mut [T], len: N) -> Option<&mut [T; N>
/// ```
///
/// Where N is an integer literal and a valid array length.
///
/// Returns `None` when length is longer than slice.
#[macro_export]
macro_rules! slice_to_array_mut {
    ($source:expr, $len:expr) => {{
        #[inline]
        fn slice_to_array_mut<T>(source: &mut [T]) -> Option<&mut [T; $len]> {
            if source.len() < $len {
                None
            } else {
                Some(unsafe {
                    slice_to_array_mut_unchecked!(source, $len)
                })
            }
        }
        
        slice_to_array_mut($source)
    }}
}

/// You can use `slice_to_array_unchecked` to turn a slice into a reference to an array without
/// bounds checking.
/// 
/// This macro is unsafe and should be called as such.
///
/// Internally, it has the signature
///
/// ```
/// unsafe fn slice_to_array_unchecked<T>(source: &[T]) -> &[T; N]
/// ```
///
/// Externally, it should be seen as having the signature
/// 
/// ```
/// unsafe slice_to_array_unchecked!(source: &[T], len: N) -> &[T; N]
/// ```
///
/// Where N is an integer literal and a valid array length.
#[macro_export]
macro_rules! slice_to_array_unchecked {
    ($source:expr, $len:expr) => {{
        #[inline]
        unsafe fn slice_to_array_unchecked<T>(source: &[T]) -> &[T; $len] {
            &*(source.as_ptr() as *const [T; $len])
        }

        slice_to_array_unchecked($source)
    }}
}

/// You can use `slice_to_array_mut_unchecked` to turn a mutable slice into a mutable reference to
/// an array without bounds checking.
/// 
/// This macro is unsafe and should be called as such.
///
/// Internally, it has the signature
///
/// ```
/// unsafe fn slice_to_array_mut_unchecked<T>(source: &mut [T]) -> &mut [T; N]
/// ```
///
/// Externally, it should be seen as having the signature
/// 
/// ```
/// unsafe slice_to_array_mut_unchecked!(source: &mut [T], len: N) -> &mut [T; N]
/// ```
///
/// Where N is an integer literal and a valid array length.
#[macro_export]
macro_rules! slice_to_array_mut_unchecked {
    ($source:expr, $len:expr) => {{
        #[inline]
        unsafe fn slice_to_array_mut_unchecked<T>(source: &mut [T]) -> &mut [T; $len] {
            &mut *(source.as_ptr() as *mut [T; $len])
        }

        slice_to_array_mut_unchecked($source)
    }}
}

/// You can use `split_to_array` to turn a slice into a reference to an array and a slice starting
/// from the end of the array.
///
/// Internally, it has the signature
///
/// ```
/// fn split_to_array<T>(source: &[T]) -> Option<(&[T; N], &[T])>
/// ```
///
/// Externally, it should be seen as having the signature
/// 
/// ```
/// slice_to_array!(source: &[T], len: N) -> Option<(&[T; N], &[T])>
/// ```
///
/// Where N is an integer literal and a valid array length.
///
/// Returns `None` when length is longer than slice.
#[macro_export]
macro_rules! split_to_array {
    ($source:expr, $len:expr) => {{
        fn split_to_array<T>(source: &[T]) -> Option<(&[T; $len], &[T])> {
            if source.len() < $len {
                None
            } else {
                let (arr, new_source) = source.split_at($len);
                Some((unsafe { slice_to_array_unchecked!(arr, $len) }, new_source))
            }
        }
        
        split_to_array($source)
    }}
}

/// You can use `split_to_array_mut` to turn a mutable slice into a mutable reference to an array
/// and a mutable slice starting from the end of the array.
///
/// Internally, it has the signature
///
/// ```
/// fn split_to_array_mut<T>(source: &mut [T]) -> Option<(&mut [T; N], &mut [T])>
/// ```
///
/// Externally, it should be seen as having the signature
/// 
/// ```
/// slice_to_array_mut!(source: &mut [T], len: N) -> Option<(&mut [T; N], &mut [T])>
/// ```
///
/// Where N is an integer literal and a valid array length.
///
/// Returns `None` when length is longer than slice.
#[macro_export]
macro_rules! split_to_array_mut {
    ($source:expr, $len:expr) => {{
        fn split_to_array_mut<T>(source: &mut [T]) -> Option<(&mut [T; $len], &mut [T])> {
            if source.len() < $len {
                None
            } else {
                let (arr, new_source) = source.split_at_mut($len);
                Some((unsafe { slice_to_array_mut_unchecked!(arr, $len) }, new_source))
            }
        }
        
        split_to_array_mut($source)
    }}
}


#[test]
fn slice_to_array_test() {
    let source = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    assert_eq!(slice_to_array!(&source[..], 10), Some(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
    assert_eq!(slice_to_array!(&source[..], 5), Some(&[1, 2, 3, 4, 5]));
    assert_eq!(slice_to_array!(&source[..5], 5), Some(&[1, 2, 3, 4, 5]));
    assert_eq!(slice_to_array!(&source[..5], 6), None);

    let source = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    assert_eq!(slice_to_array!(&source[..], 10), Some(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
    assert_eq!(slice_to_array!(&source[..], 5), Some(&[1, 2, 3, 4, 5]));
    assert_eq!(slice_to_array!(&source[..5], 5), Some(&[1, 2, 3, 4, 5]));
    assert_eq!(slice_to_array!(&source[..5], 6), None);
}

#[test]
fn slice_to_array_mut_test() {
    let mut source = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    {
        if let Some(arr) = slice_to_array_mut!(&mut source[3..7], 4) {
            arr[3] = 100;
        }
    }
    assert_eq!(source, [1, 2, 3, 4, 5, 6, 100, 8, 9, 10]);
}


#[test]
fn split_to_array_test() {
    let source = [1, 2, 3, 4, 5];
    assert_eq!(split_to_array!(&source[..], 3), Some((&[1, 2, 3], &source[3..])));
    assert_eq!(split_to_array!(&source[..], 6), None);
}

#[test]
fn split_to_array_mut_test() {
    let mut source = [1, 2, 3, 4, 5];
    {
        if let Some((arr, end)) = split_to_array_mut!(&mut source, 3) {
            arr[1] = 100;
            end[1] = 200;
        }
    }
    assert_eq!(source, [1, 100, 3, 4, 200]);
}
