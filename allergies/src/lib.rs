use std::collections::HashMap;

pub struct Allergies(u32);

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
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
static SCORE_MAP : HashMap<Allergen, u32> = vec![
    (Allergen::Eggs, 1),
    (Allergen::Peanuts, 2),
    (Allergen::Shellfish, 4),
    (Allergen::Strawberries, 8),
    (Allergen::Tomatoes, 16),
    (Allergen::Chocolate, 32),
    (Allergen::Pollen, 64),
    (Allergen::Cats, 128),
].iter().cloned().collect();

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies(score)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.0 & *SCORE_MAP.get(allergen).unwrap() != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        SCORE_MAP
            .keys()
            .filter(|x| {
                self.is_allergic_to(*x)
            })
            .cloned()
            .collect()
    }
}
