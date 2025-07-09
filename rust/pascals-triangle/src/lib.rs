pub struct PascalsTriangle {
    triangle: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut triangle: Vec<Vec<u32>> = Vec::new();
        if row_count > 0 {
            triangle.push(vec![1]);

            for i in 2..=row_count {
                let previous_row = &triangle[(i - 2) as usize];
                let mut row: Vec<u32> = vec![1];

                for j in 1..(i - 1) {
                    row.push(previous_row[(j - 1) as usize] + previous_row[j as usize]);
                }

                row.push(1);
                triangle.push(row);
            }
        }

        PascalsTriangle { triangle: triangle }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.triangle.clone()
    }
}
