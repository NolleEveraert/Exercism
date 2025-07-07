pub fn annotate(garden: &[&str]) -> Vec<String> {
    if garden.is_empty() {
        return Vec::new();
    }

    let mut filled_garden: Vec<String> = garden.iter().map(|row| " ".repeat(row.len())).collect();

    let max_x: usize = garden[0].len();
    let max_y: usize = garden.len();

    for y in 0..garden.len() {
        for x in 0..garden[0].len() {
            let current_tile: char = garden[y].as_bytes()[x] as char;

            let flower_count: u64 = get_neighbouring_flowers(garden, x, y, max_x, max_y);

            if current_tile == '*' {
                filled_garden[y].replace_range(x..x + 1, "*");
            } else if flower_count == 0 {
                filled_garden[y].replace_range(x..x + 1, " ");
            } else {
                filled_garden[y].replace_range(x..x + 1, &flower_count.to_string());
            }
        }
    }

    return filled_garden;
}

pub fn get_neighbouring_flowers(field: &[&str], x: usize, y: usize, max_x: usize, max_y: usize) -> u64 {
    let mut flower_count = 0;

    // Define all 8 neighboring positions relative to current position
    let neighbor_offsets = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

    // Check each neighboring position
    for (dx, dy) in neighbor_offsets {
        let neighbor_x: i32 = x as i32 + dx;
        let neighbor_y: i32 = y as i32 + dy;

        // Check if the neighbor position is within bounds
        let is_valid_x: bool = neighbor_x >= 0 && neighbor_x < max_x as i32;
        let is_valid_y: bool = neighbor_y >= 0 && neighbor_y < max_y as i32;

        if is_valid_x && is_valid_y {
            let tile: char = field[neighbor_y as usize].as_bytes()[neighbor_x as usize] as char;
            if tile == '*' {
                flower_count += 1;
            }
        }
    }

    return flower_count;
}
