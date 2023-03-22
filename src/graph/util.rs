use super::traits::Direction;

#[inline]
fn to_flat_square_matrix_position(row: usize, column: usize, width: usize) -> usize {
    row * width + column
}

/// Converts a row and column index to a position in a lower triangular matrix.
#[inline]
fn to_lower_triangular_matrix_position(row: usize, column: usize) -> usize {
    let (row, column) = if row > column {
        (row, column)
    } else {
        (column, row)
    };
    (row * (row + 1)) / 2 + column
}

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
        extend_flat_square_matrix(node_adjacencies, old_node_capacity, new_capacity)
    } else {
        extend_lower_triangular_matrix(node_adjacencies, new_capacity)
    }
}

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

#[inline]
fn extend_lower_triangular_matrix<T: Default>(node_adjacencies: &mut Vec<T>, new_capacity: usize) {
    let max_node = new_capacity - 1;
    let max_pos = to_lower_triangular_matrix_position(max_node, max_node);
    ensure_len(node_adjacencies, max_pos + 1);
}

/// Grow a Vec by appending the type's default value until the `size` is reached.
fn ensure_len<T: Default>(v: &mut Vec<T>, size: usize) {
    if let Some(n) = size.checked_sub(v.len()) {
        v.reserve(n);
        for _ in 0..n {
            v.push(T::default());
        }
    }
}
