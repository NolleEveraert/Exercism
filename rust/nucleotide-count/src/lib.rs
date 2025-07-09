use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if nucleotide != 'A' && nucleotide != 'C' && nucleotide != 'G' && nucleotide != 'T' {
        return Err(nucleotide);
    }

    let counts = nucleotide_counts(dna)?;
    let amount = counts.get(&nucleotide).copied().unwrap_or(0);
    Ok(amount)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    if let Some(invalid) = dna.chars().find(|&c| c != 'A' && c != 'C' && c != 'G' && c != 'T') {
        return Err(invalid);
    }

    let mut result: HashMap<char, usize> = HashMap::new();
    result.insert('A', 0);
    result.insert('C', 0);
    result.insert('G', 0);
    result.insert('T', 0);

    for char in dna.chars() {
        if let Some(count) = result.get_mut(&char) {
            *count += 1;
        }
    }

    Ok(result)
}
