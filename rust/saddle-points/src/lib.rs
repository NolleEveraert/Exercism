pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut potential_trees: Vec<(usize, usize)> = Vec::new();

    for index_row in 0..input.len() {
        let row = &input[index_row];

        for index_col in 0..row.len() {
            let center = row[index_col];

            // Check if center is maximum in its row
            let row_max = row.iter().max().unwrap();
            if center != *row_max {
                continue;
            }

            // Check if center is minimum in its column
            let col_min = (0..input.len()).map(|r| input[r][index_col]).min().unwrap();
            if center != col_min {
                continue;
            }

            potential_trees.push((index_row, index_col));
        }
    }

    potential_trees
}
