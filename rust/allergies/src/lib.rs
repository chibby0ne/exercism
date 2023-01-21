pub struct Allergies {
    allergens: Vec<Allergen>,
}

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

impl Allergies {
    fn score_to_allergen(score: u32) -> Option<Allergen> {
        match score {
            1 => Some(Allergen::Eggs),
            2 => Some(Allergen::Peanuts),
            4 => Some(Allergen::Shellfish),
            8 => Some(Allergen::Strawberries),
            16 => Some(Allergen::Tomatoes),
            32 => Some(Allergen::Chocolate),
            64 => Some(Allergen::Pollen),
            128 => Some(Allergen::Cats),
            _ => None,
        }
    }

    fn find_allergens(score: u32) -> Vec<Allergen> {
        let mut vec: Vec<Allergen> = Vec::new();
        if score == 0 {
            return vec;
        }

        let mut mut_score = score;
        // We are only interested in the first 8 bits of the score since they would represent the
        // scores mapped to the known allergens
        for i in 0..8 {
            let bit = mut_score & 1;
            if let Some(allergy) = Allergies::score_to_allergen(bit * 2u32.pow(i)) {
                vec.push(allergy);
            }
            mut_score >>= 1;
        }
        vec
    }

    pub fn new(score: u32) -> Self {
        Self {
            allergens: Allergies::find_allergens(score),
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergens.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergens.to_vec()
    }
}
