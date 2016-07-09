pub struct Allergies {
    bitmap: u8,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Copy)]
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

use Allergen::*;
static ALLERGENS: &'static [Allergen; 8] = &[
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
];

impl Allergies {
    pub fn new(bitmap: u8) -> Allergies {
        Allergies {
            bitmap: bitmap,
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        (self.bitmap & match allergen {
            &Eggs         => 0b00000001,
            &Peanuts      => 0b00000010,
            &Shellfish    => 0b00000100,
            &Strawberries => 0b00001000,
            &Tomatoes     => 0b00010000,
            &Chocolate    => 0b00100000,
            &Polen        => 0b01000000,
            //&Cats         => 0b10000000,
        }) != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        ALLERGENS.iter()
            .filter(|allergen| self.is_allergic_to(allergen))
            .map(|a| a.clone())
            .collect()
    }
}


