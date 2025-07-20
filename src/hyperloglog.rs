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

        // Initialize the hash function. P is a prime number close to the amount of 
        // letter in the alphabet (31). m is a really big prime number 
        // Probelm to address: PolRolHF needs long strings to increase.
        // For the way it is like now, one letter words will have very similar hashes
        // amt_register is the starting number (a sort of salt) because it ensure that
        // there is a one after the first lnm bits. Otherwise, get_luckiness would o
        let hashing_function = PolRolHF::new(31, 9 + pow(10,9), amt_register as u128);

        // compute log2(m) ones at the beginning to reuse it freely afterwards
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

    // This function counts the amount of zero bits in the binary
    // representation of the input number, starting from the 
    // less significant ones
    fn get_luckiness(mut number: u128)-> u8{
        if number == 0 {
            // prvent overflow 
            return 128;
        }
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

        // The registry number is given by the first lnm bits of the hash number.
        let registry = hashed_value & (pow(2, self.lnm) - 1);
        
        // println!("m: {}, lnm: {}", self.m, self.lnm);
        // println!("Hash: {:b}. \n regi: {:b}, {}", hashed_value, registry, registry);
        // println!("shifted: {:b}", hashed_value >> self.lnm);
        let luckiness = Self::get_luckiness(hashed_value >> self.lnm);

        // Update registry with improved value
        self.bests[registry as usize] = self.bests[registry as usize].max(luckiness + 1);
    }

    // Computes harmonic mean on the vector containing the best result.
    fn harmonic_mean (bests: &Vec<u8>) -> f64{
        let reciprocal_total:f64 = bests.iter().map(|&best| {
            2f64.powi(best as i32 * -1)
        }).sum();
        return reciprocal_total.recip();
    }

    pub fn count(&self)-> usize {
        let float_m: f64 = self.m as f64;
        let amt_empty_registers = self.bests.iter().filter(|&reg| *reg == 0).count();
        if amt_empty_registers > 0 {
            println!("Returning Linear Counting Estimation");
            println!("Empty registers: {}", amt_empty_registers);
            return (float_m * (float_m / amt_empty_registers as f64).log2().floor()) as usize;
        } else {
            println!("Returning HyperLogLog Estimation");
            let z = Self::harmonic_mean(&self.bests);
            let alpha = (0.7213)/ (1 as f64 + (1.079 / float_m));
            println!("alpha: {}, z: {} float_m: {}", alpha, z, float_m);

            return (alpha * z * float_m * float_m).floor() as usize;
        }
    }
}
