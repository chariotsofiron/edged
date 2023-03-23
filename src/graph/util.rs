//! Util functions for adjacency matrix graphs.
use crate::util::ensure_len;

use super::traits::Direction;

/// Converts a row and column index to a position in a flat square matrix.
#[inline]
const fn to_flat_square_matrix_position(row: usize, column: usize, width: usize) -> usize {
    row * width + column
}

/// Converts a row and column index to a position in a lower triangular matrix.
#[inline]
const fn to_lower_triangular_matrix_position(row: usize, column: usize) -> usize {
    let (long, short) = if row > column {
        (row, column)
    } else {
        (column, row)
    };
    (long * (long + 1)) / 2 + short
}

/// Converts a row and column index to a position in a matrix.
#[must_use]
#[inline]
pub fn to_linear_matrix_position<Ty: Direction>(row: usize, column: usize, width: usize) -> usize {
    if Ty::is_directed() {
        to_flat_square_matrix_position(row, column, width)
    } else {
        to_lower_triangular_matrix_position(row, column)
    }
}

/// Extends a vector representing a square matrix to support holding `new_capacity` nodes.
/// `old_node_capacity` is the number of nodes the matrix currently supports.
/// `new_capacity` is the number of nodes the matrix should support after this function returns.
#[inline]
pub fn extend_linearized_matrix<Ty: Direction, T: Default>(
    node_adjacencies: &mut Vec<T>,
    old_node_capacity: usize,
    new_capacity: usize,
) {
    if Ty::is_directed() {
        extend_flat_square_matrix(node_adjacencies, old_node_capacity, new_capacity);
    } else {
        extend_lower_triangular_matrix(node_adjacencies, new_capacity);
    }
}

/// Extend a vector representing a square matrix to support holding `new_capacity` nodes.
#[inline]
fn extend_flat_square_matrix<T: Default>(
    node_adjacencies: &mut Vec<T>,
    old_node_capacity: usize,
    new_node_capacity: usize,
) {
    ensure_len(node_adjacencies, new_node_capacity.pow(2));
    for c in (1..old_node_capacity).rev() {
        let pos = c * old_node_capacity;
        let new_pos = c * new_node_capacity;
        // Move the slices directly if they do not overlap with their new position
        if pos + old_node_capacity <= new_pos {
            debug_assert!(pos + old_node_capacity < node_adjacencies.len());
            debug_assert!(new_pos + old_node_capacity < node_adjacencies.len());
            let ptr = node_adjacencies.as_mut_ptr();
            // SAFETY: pos + old_node_capacity <= new_pos, so this won't overlap
            unsafe {
                let old = ptr.add(pos);
                let new = ptr.add(new_pos);
                core::ptr::swap_nonoverlapping(old, new, old_node_capacity);
            }
        } else {
            for i in (0..old_node_capacity).rev() {
                node_adjacencies.as_mut_slice().swap(pos + i, new_pos + i);
            }
        }
    }
}

/// Extends a vector representing a lower triangular matrix to support holding `new_capacity` nodes.
#[inline]
fn extend_lower_triangular_matrix<T: Default>(node_adjacencies: &mut Vec<T>, new_capacity: usize) {
    let blah = (new_capacity * (new_capacity - 1)) / 2 + new_capacity + 1;
    ensure_len(node_adjacencies, blah);
}

