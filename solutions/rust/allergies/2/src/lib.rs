pub struct Allergies {
    score: u32
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergen  {
    pub fn from_discriminant(allergy: u32) -> Self {
        match allergy {
            1 => Allergen::Eggs,
            2 => Allergen::Peanuts,
            4 => Allergen::Shellfish,
            8 => Allergen::Strawberries,
            16 => Allergen::Tomatoes,
            32 => Allergen::Chocolate,
            64 => Allergen::Pollen,
            _ => Allergen::Cats,
        }
    }
}


impl Allergies {
    pub fn new(score: u32) -> Self {
        Self {
            score
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.score & (allergen.clone() as u32) == allergen.clone() as u32 
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut score = self.score;
        let mut result = vec!();
        score &= 0b11111111;
        let mut allergen: u32 = 1;
        while allergen <= 128 {
            if score & allergen == allergen {
                result.push(Allergen::from_discriminant(allergen))
            }
            allergen *= 2;
        }
        result
        }
}

// impl Allergies {
//     pub fn new(score: u32) -> Self {
//         Self {
//             score
//         }
//     }

//     pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
//         self.allergies().contains(allergen)
//     }

//     fn nearest_allergy(&self, i: u32) -> (Option<Allergen>, u32) {
//         let mut c = 1;
//         while c <= i {
//             c *= 2;
//         }
//         c /= 2;
//         let allergy = match c {
//             1 => Some(Allergen::Eggs),
//             2 => Some(Allergen::Peanuts),
//             4 => Some(Allergen::Shellfish),
//             8 => Some(Allergen::Strawberries),
//             16 => Some(Allergen::Tomatoes),
//             32 => Some(Allergen::Chocolate),
//             64 => Some(Allergen::Pollen),
//             128 => Some(Allergen::Cats),
//             _ => None
//         };
//         (allergy, c)
//     }

//     pub fn allergies(&self) -> Vec<Allergen> {
//         let mut score = self.score;
//         let mut result = vec!();
//         while score > 0 {
//             let (a, s) = self.nearest_allergy(score);
//             score -= s;
//             if let Some(allergy) = a {
//                 result.insert(0, allergy);
//             }
//         }
//         result
//     }
// }
