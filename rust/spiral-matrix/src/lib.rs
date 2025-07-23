pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    if size == 0 {
        return Vec::new();
    }

    let size: usize = size as usize;
    let mut matrix: Vec<Vec<u32>> = vec![vec![0u32; size]; size];

    let mut num: u32 = 1u32;
    let mut top: usize = 0;
    let mut bottom: usize = size - 1;
    let mut left: usize = 0;
    let mut right = size - 1;

    while top <= bottom && left <= right {
        // Fill top row (left to right)
        for col in left..=right {
            matrix[top][col] = num;
            num += 1;
        }
        top += 1;

        // Fill right column (top to bottom)
        for row in top..=bottom {
            matrix[row][right] = num;
            num += 1;
        }
        right = right.saturating_sub(1);

        // Fill bottom row (right to left)
        if top <= bottom {
            for col in (left..=right).rev() {
                matrix[bottom][col] = num;
                num += 1;
            }
            bottom = bottom.saturating_sub(1);
        }

        // Fill left column (bottom to top)
        if left <= right {
            for row in (top..=bottom).rev() {
                matrix[row][left] = num;
                num += 1;
            }
            left += 1;
        }
    }

    matrix
}
