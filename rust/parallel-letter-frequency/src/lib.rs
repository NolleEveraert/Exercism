use std::{collections::HashMap, thread};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let chunk_size = if worker_count == 0 {
        1
    } else {
        input.len().div_ceil(worker_count)
    };

    let chunks = if chunk_size == 0 {
        input.chunks(1)
    } else {
        input.chunks(chunk_size)
    };

    let mut threads: Vec<thread::JoinHandle<_>> = Vec::new();
    for chunk in chunks {
        let owned_chunk: Vec<String> = chunk.iter().map(|s| s.to_string()).collect();
        threads.push(thread::spawn(move || count_characters(&owned_chunk)));
    }

    let results: Vec<HashMap<char, usize>> = threads.into_iter().map(|handle| handle.join().unwrap()).collect();

    results.into_iter().fold(HashMap::new(), |mut acc, map| {
        for (key, count) in map {
            acc.entry(key)
                .and_modify(|existing| *existing += count)
                .or_insert(count);
        }
        acc
    })
}

fn count_characters(chunk: &[String]) -> HashMap<char, usize> {
    let mut frequency_table: HashMap<char, usize> = HashMap::new();

    for s in chunk {
        for char in s.chars() {
            if char.is_alphabetic() {
                *frequency_table.entry(char.to_ascii_lowercase()).or_insert(0) += 1;
            }
        }
    }

    frequency_table
}
