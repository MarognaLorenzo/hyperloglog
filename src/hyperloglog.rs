use std::cmp::max;
use rand::Rng; use num_traits::pow;

use crate::hyperloglog::hashing::{Hashing, PolRolHF};

mod hashing;
pub struct HyperLogLog {
    bests:Vec<u8>,
    hashing_functions: Vec<PolRolHF>,
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
        
        HyperLogLog {hashing_functions, bests}
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

    pub fn get_single_words(&self)-> f64 {
        let something = self.bests.iter().map( |&best| pow(2, best as usize + 1) as f64);
        let result: f64 = something.sum();
        return result / self.bests.len() as f64;
    }
}
