use num_traits::{pow, Pow};
use rand::random_range;

use crate::hyperloglog::hashing::{Hashing, PolRolHF};

mod hashing;
pub struct HyperLogLog {
    bests:Vec<u8>,
    hashing_function: PolRolHF,
    m: usize,
    lnm: usize,
}

impl HyperLogLog {
    pub fn new (amt_register: usize) -> HyperLogLog {
        let mut bests = vec![];
        bests.resize(amt_register, 0 as u8);
        let hashing_function = PolRolHF::new(31, 9 + pow(10,9), random_range(100..pow(10,9)));
        let lnm = (amt_register as f32).log2().floor() as usize;
        HyperLogLog {hashing_function, bests, lnm, m: amt_register}
    }

    pub fn from (self, registry: Vec<u8>) -> HyperLogLog {
        HyperLogLog { 
            bests: registry,
            hashing_function: self.hashing_function,
            m: self.m,
            lnm: self.lnm
        }
    }
    fn get_luckiness(mut number: u128)-> u8{
        let mut result: u8 = 0;
        while number % 2 == 0 {
            result += 1;
            number /= 2;
        }
        return result;
    }

    pub fn merge(self, other: HyperLogLog) -> HyperLogLog {
        // Merging results in taking the max from each registy
        let new_bests = self.bests.iter().zip(other.bests).map(|(&a,b)| if a > b {a} else {b}).collect();
        self.from(new_bests)
    }

    pub fn add(&mut self, word: String){
        let hashed_value = self.hashing_function.hash(&word);
        let registry = hashed_value & (pow(2, self.lnm) - 1);
        
        // println!("m: {}, lnm: {}", self.m, self.lnm);
        // println!("Hash: {:b}. \n regi: {:b}, {}", hashed_value, registry, registry);
        // println!("shifted: {:b}", hashed_value >> self.lnm);
        let luckiness = Self::get_luckiness(hashed_value >> self.lnm);
        if luckiness >= 14 {
            println!("---------------------");
            println!("WORD: {}", word);
            println!("Hash: {:b}", hashed_value);
            println!("Luckiness: {}", luckiness);
            println!("Registry: {}", registry);
            println!("Content: {}", self.bests[registry as usize]);
        }
        self.bests[registry as usize] = self.bests[registry as usize].max(luckiness);
    }

    // Computes harmonic mean on the vector containing the best result.
    fn harmonic_mean (bests: &Vec<u8>) -> f64{
        let reciprocal_total:f64 = bests.iter().map(|&best| {
            let exponent: i32 = - (best as i32 +1);
            2f64.powi(exponent)
        }).sum();
        let harmonic_average: f64 = bests.len() as f64 / reciprocal_total;
        return harmonic_average;
    }

    pub fn count(&self)-> f64 {
        let float_m: f64 = self.m as f64;
        let amt_empty_registers = self.bests.iter().filter(|&reg| *reg == 0).count();
        if amt_empty_registers > 0 {
            println!("Returning Linear Counting Estimation");
            println!("Empty registers: {}", amt_empty_registers);
            return float_m * (float_m / amt_empty_registers as f64).log2();
        } else {
            println!("Returning HyperLogLog Estimation");
            let z = Self::harmonic_mean(&self.bests);
            let alpha = (0.7213)/ (1 as f64 + (1.079 / float_m));
            println!("alpha: {}, z: {} float_m: {}", alpha, z, float_m);

            for reg in &self.bests {
                println!("Reg: {}", reg);
            }
            return alpha * z * float_m * float_m;
        }
    }
}
