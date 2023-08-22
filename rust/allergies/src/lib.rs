pub struct Allergies {
    value: u32,
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

impl From<u32> for Allergen {
    fn from(value: u32) -> Self {
        match value {
            1 => Self::Eggs,
            2 => Self::Peanuts,
            4 => Self::Shellfish,
            8 => Self::Strawberries,
            16 => Self::Tomatoes,
            32 => Self::Chocolate,
            64 => Self::Pollen,
            128 => Self::Cats,
            _ => unreachable!(),
        }
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self { value: score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let allergen_number: u32 = allergen.clone() as u32;

        self.value & allergen_number > 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut res = vec![];

        if self.value & Allergen::Eggs as u32 > 0 {
            res.push(Allergen::Eggs);
        }
        if self.value & Allergen::Peanuts as u32 > 0 {
            res.push(Allergen::Peanuts);
        }
        if self.value & Allergen::Shellfish as u32 > 0 {
            res.push(Allergen::Shellfish);
        }
        if self.value & Allergen::Strawberries as u32 > 0 {
            res.push(Allergen::Strawberries);
        }
        if self.value & Allergen::Tomatoes as u32 > 0 {
            res.push(Allergen::Tomatoes);
        }
        if self.value & Allergen::Chocolate as u32 > 0 {
            res.push(Allergen::Chocolate);
        }
        if self.value & Allergen::Pollen as u32 > 0 {
            res.push(Allergen::Pollen);
        }
        if self.value & Allergen::Cats as u32 > 0 {
            res.push(Allergen::Cats);
        }

        res
    }
}
