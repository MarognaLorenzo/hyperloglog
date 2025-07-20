use rand::Rng; use num_traits::{pow, Pow};

use crate::hyperloglog::hashing::{Hashing, PolRolHF};

enum Avg {
    Std,
    Harmonic,
}
mod hashing;
pub struct HyperLogLog {
    bests:Vec<u8>,
    hashing_functions: Vec<PolRolHF>,
    avereging_strategy: Avg,
}

impl HyperLogLog {
    pub fn new (amt_register: usize) -> HyperLogLog {
        let mut hashing_functions = vec![];
        let mut bests = vec![];
        bests.resize(amt_register, 0 as u8);
        let mut generator = rand::rng();
        for _ in 0..amt_register {
            let salt = generator.random_range(0..pow(10,9) + 9);
            hashing_functions.push(PolRolHF::new(31, 9 + pow(10,9), salt));
        }
        
        HyperLogLog {hashing_functions, bests, avereging_strategy: Avg::Std}
    }
    fn get_luckiness(mut number: u128)-> u8{
        let mut result: u8 = 0;
        while number % 2 == 0 {
            result += 1;
            number /= 2;
        }
        return result;
    }

    pub fn receive(&mut self, word: String){
        for (function, best) in self.hashing_functions.iter_mut().zip(&mut self.bests) {
            let hashed_value = function.hash(word.clone());
            let luckiness = Self::get_luckiness(hashed_value);
            if luckiness > *best {
                *best = luckiness;
            }
        }
    }

    fn std_average( bests: & Vec<u8>) -> f64 {
        let partial_total :f64 = bests.iter().map( |&best| 2f64.pow(best + 1)).sum();
        let result: f64 = partial_total / bests.len() as f64;
        return result;
    }

    fn harmonic_average (bests: &Vec<u8>) -> f64{
        let reciprocal_total:f64 = bests.iter().map(|&best| {
            let exponent: i32 = - (best as i32 +1);
            2f64.powi(exponent)
        }).sum();
        let harmonic_average: f64 = bests.len() as f64 / reciprocal_total;
        return harmonic_average;
    }

    pub fn set_harmonic(&mut self) {
        self.avereging_strategy = Avg::Harmonic
    }

    pub fn get_single_words(&self)-> f64 {
        let result = match self.avereging_strategy {
            Avg::Std => Self::std_average(&self.bests),
            Avg::Harmonic => Self::harmonic_average(&self.bests),

        };
        result
    }
}
