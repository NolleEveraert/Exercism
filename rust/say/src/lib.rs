pub fn encode(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }

    let chunks = break_into_chunks(n);
    chunks_to_words(chunks)
}

fn break_into_chunks(n: u64) -> Vec<u64> {
    let mut chunks = Vec::new();
    let mut remaining = n;

    while remaining > 0 {
        chunks.push(remaining % 1000);
        remaining /= 1000;
    }

    chunks.reverse();
    chunks
}

fn chunks_to_words(chunks: Vec<u64>) -> String {
    let scale_words = [
        "",
        "thousand",
        "million",
        "billion",
        "trillion",
        "quadrillion",
        "quintillion",
    ];
    let mut result = Vec::new();

    for (i, &chunk) in chunks.iter().enumerate() {
        if chunk == 0 {
            continue;
        }

        let chunk_words = chunk_to_words(chunk);
        let scale_index = chunks.len() - 1 - i;

        if scale_index > 0 && scale_index < scale_words.len() {
            result.push(format!("{} {}", chunk_words, scale_words[scale_index]));
        } else {
            result.push(chunk_words);
        }
    }

    result.join(" ")
}

fn chunk_to_words(n: u64) -> String {
    let mut parts = Vec::new();

    if n >= 100 {
        let hundreds = n / 100;
        parts.push(format!("{} hundred", tens(hundreds)));
    }

    let remainder = n % 100;
    if remainder > 0 {
        parts.push(tens(remainder));
    }

    parts.join(" ")
}

fn tens(n: u64) -> String {
    match n {
        0..=10 => match n {
            0 => "zero",
            1 => "one",
            2 => "two",
            3 => "three",
            4 => "four",
            5 => "five",
            6 => "six",
            7 => "seven",
            8 => "eight",
            9 => "nine",
            10 => "ten",
            _ => unreachable!(),
        }
        .to_string(),
        11..=19 => match n {
            11 => "eleven",
            12 => "twelve",
            13 => "thirteen",
            14 => "fourteen",
            15 => "fifteen",
            16 => "sixteen",
            17 => "seventeen",
            18 => "eighteen",
            19 => "nineteen",
            _ => unreachable!(),
        }
        .to_string(),
        20..=99 => {
            let tens_word = match n / 10 {
                2 => "twenty",
                3 => "thirty",
                4 => "forty",
                5 => "fifty",
                6 => "sixty",
                7 => "seventy",
                8 => "eighty",
                9 => "ninety",
                _ => unreachable!(),
            };

            match n % 10 {
                0 => tens_word.to_string(),
                ones => format!(
                    "{}-{}",
                    tens_word,
                    match ones {
                        1 => "one",
                        2 => "two",
                        3 => "three",
                        4 => "four",
                        5 => "five",
                        6 => "six",
                        7 => "seven",
                        8 => "eight",
                        9 => "nine",
                        _ => unreachable!(),
                    }
                ),
            }
        }
        _ => String::new(),
    }
}
