#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

const ALLERGENS: [Allergen; 8] = [
    Allergen::Eggs,
    Allergen::Peanuts,
    Allergen::Shellfish,
    Allergen::Strawberries,
    Allergen::Tomatoes,
    Allergen::Chocolate,
    Allergen::Pollen,
    Allergen::Cats,
];

pub struct Allergies {
    score: u32,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let index: u32 = *allergen as u32;
        let combination = (1 << index) & self.score;
        return combination != 0;
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut result: Vec<Allergen> = Vec::new();

        for index in 0..8 {
            if (1 << index) & self.score != 0 {
                result.push(ALLERGENS[index]);
            }
        }

        return result;
    }
}
