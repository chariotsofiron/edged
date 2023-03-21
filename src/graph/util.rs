use super::graph_type::GraphType;


/// Grows a vector representing a square matrix to a specific dimension.
/// `exact` indicates whether the capacity should be exactly `new_capacity` or
/// if it can be rounded up to the next power of two.
#[inline]
fn extend_flat_square_matrix<T: Default>(
    adjacencies: &mut Vec<T>,
    old_capacity: usize,
    new_capacity: usize,
) -> usize {
    ensure_len(adjacencies, new_capacity.pow(2));
    for c in (1..old_capacity).rev() {
        let pos = c * old_capacity;
        let new_pos = c * new_capacity;
        // Move the slices directly if they do not overlap with their new position
        if pos + old_capacity <= new_pos {
            debug_assert!(pos + old_capacity < adjacencies.len());
            debug_assert!(new_pos + old_capacity < adjacencies.len());
            let ptr = adjacencies.as_mut_ptr();
            // SAFETY: pos + old_node_capacity <= new_pos, so this won't overlap
            unsafe {
                let old = ptr.add(pos);
                let new = ptr.add(new_pos);
                core::ptr::swap_nonoverlapping(old, new, old_capacity);
            }
        } else {
            for i in (0..old_capacity).rev() {
                adjacencies.as_mut_slice().swap(pos + i, new_pos + i);
            }
        }
    }
    new_capacity
}

#[inline]
fn extend_lower_triangular_matrix<T: Default>(
    adjacencies: &mut Vec<T>,
    new_capacity: usize,
) -> usize {
    let max_node = new_capacity - 1;
    let max_pos = to_lower_triangular_matrix_position(max_node, max_node);
    ensure_len(adjacencies, max_pos + 1);
    new_capacity
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

#[inline]
pub fn extend_linear_matrix<Ty: GraphType, T: Default>(
    adjacencies: &mut Vec<T>,
    capacity: usize,
) -> usize {
    let old_capacity = adjacencies.capacity();
    println!("new_cap: {new_capacity}, {old_capacity}, {}", adjacencies.capacity());
    if old_capacity >= capacity {
        return old_capacity;
    }
    if Ty::is_directed() {
        extend_flat_square_matrix(adjacencies, old_capacity, capacity)
    } else {
        extend_lower_triangular_matrix(adjacencies, capacity)
    }
}

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
pub fn to_linearized_matrix_position<Ty: GraphType>(row: usize, column: usize, width: usize) -> usize {
    if Ty::is_directed() {
        to_flat_square_matrix_position(row, column, width)
    } else {
        to_lower_triangular_matrix_position(row, column)
    }
}