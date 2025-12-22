pub struct Allergies {
    score: u32
}

#[derive(Debug, PartialEq, Eq)]
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
    pub fn new(score: u32) -> Self {
        Self {
            score
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies().contains(allergen)
    }

    fn nearest_allergy(&self, i: u32) -> (Option<Allergen>, u32) {
        let mut c = 1;
        while c <= i {
            c *= 2;
        }
        c /= 2;
        let allergy = match c {
            1 => Some(Allergen::Eggs),
            2 => Some(Allergen::Peanuts),
            4 => Some(Allergen::Shellfish),
            8 => Some(Allergen::Strawberries),
            16 => Some(Allergen::Tomatoes),
            32 => Some(Allergen::Chocolate),
            64 => Some(Allergen::Pollen),
            128 => Some(Allergen::Cats),
            _ => None
        };
        (allergy, c)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut score = self.score;
        let mut result = vec!();
        while score > 0 {
            let (a, s) = self.nearest_allergy(score);
            score -= s;
            if let Some(allergy) = a {
                result.insert(0, allergy);
            }
        }
        result
    }
}
