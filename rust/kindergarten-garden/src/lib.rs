pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let index = student_index(student);
    let grid: Vec<Vec<char>> = diagram.lines().map(|line| line.chars().collect()).collect();
    let mut result: Vec<&'static str> = Vec::new();

    result.push(get_plant_name(grid[0][index * 2]));
    result.push(get_plant_name(grid[0][index * 2 + 1]));
    result.push(get_plant_name(grid[1][index * 2]));
    result.push(get_plant_name(grid[1][index * 2 + 1]));
    return result;
}

fn student_index(student: &str) -> usize {
    match student {
        "Alice" => 0,
        "Bob" => 1,
        "Charlie" => 2,
        "David" => 3,
        "Eve" => 4,
        "Fred" => 5,
        "Ginny" => 6,
        "Harriet" => 7,
        "Ileana" => 8,
        "Joseph" => 9,
        "Kincaid" => 10,
        "Larry" => 11,
        _ => panic!("Unknown student"),
    }
}

fn get_plant_name(plant: char) -> &'static str {
    match plant {
        'G' => "grass",
        'C' => "clover",
        'R' => "radishes",
        'V' => "violets",
        _ => panic!("Unknown plant"),
    }
}
