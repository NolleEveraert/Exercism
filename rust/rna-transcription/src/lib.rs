#[derive(Debug, PartialEq, Eq)]
enum NucleotidesDNA {
    G,
    C,
    T,
    A,
}

#[derive(Debug, PartialEq, Eq)]
enum NucleotidesRNA {
    C,
    G,
    A,
    U,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    transcription: Vec<NucleotidesDNA>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    transcription: Vec<NucleotidesRNA>,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let mut transcription: Vec<NucleotidesDNA> = Vec::new();
        for (i, n) in dna.chars().enumerate() {
            let nucleotide = match n.to_ascii_uppercase() {
                'G' => NucleotidesDNA::G,
                'C' => NucleotidesDNA::C,
                'T' => NucleotidesDNA::T,
                'A' => NucleotidesDNA::A,
                _ => return Err(i),
            };
            transcription.push(nucleotide);
        }

        Ok(Dna {
            transcription: transcription,
        })
    }

    pub fn into_rna(self) -> Rna {
        Rna {
            transcription: self.transcription.iter().map(|n| complement(n)).collect(),
        }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let mut transcription: Vec<NucleotidesRNA> = Vec::new();
        for (i, n) in rna.chars().enumerate() {
            let nucleotide = match n.to_ascii_uppercase() {
                'G' => NucleotidesRNA::G,
                'C' => NucleotidesRNA::C,
                'U' => NucleotidesRNA::U,
                'A' => NucleotidesRNA::A,
                _ => return Err(i),
            };
            transcription.push(nucleotide);
        }

        Ok(Rna {
            transcription: transcription,
        })
    }
}

fn complement(nucleotide: &NucleotidesDNA) -> NucleotidesRNA {
    match nucleotide {
        NucleotidesDNA::G => NucleotidesRNA::C,
        NucleotidesDNA::C => NucleotidesRNA::G,
        NucleotidesDNA::T => NucleotidesRNA::A,
        NucleotidesDNA::A => NucleotidesRNA::U,
    }
}
