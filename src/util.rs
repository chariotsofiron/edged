//! Utility functions.

/// Grow a Vec by appending the type's default value until the `size` is reached.
pub(crate) fn ensure_len<T: Default>(v: &mut Vec<T>, size: usize) {
    if let Some(n) = size.checked_sub(v.len()) {
        v.reserve(n);
        for _ in 0..n {
            v.push(T::default());
        }
    }
}
