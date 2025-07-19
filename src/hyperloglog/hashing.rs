use num_traits::pow;

pub trait Hashing {
    fn hash(&self, input: String) -> u128;
}
pub struct PolRolHF{
    p: u128,
    m: u128,
}

impl PolRolHF {
    pub fn new(p: u128, m: u128) -> PolRolHF {
        PolRolHF {p, m}
    }
}

impl Hashing for PolRolHF {
    fn hash(&self, input: String) -> u128 {
        let word_as_bytes = input.as_bytes();
        let mut res: u128 = 0;
        // println!("{}", input);

        let mut powering = 1;

        for i in 0..input.len() {
            let char_val: &u8 = word_as_bytes.get(i).unwrap();
            res += *char_val as u128 * powering;
            powering *= self.p;
            powering %= self.m;
            res %= self.m;
            // println!("{}", res);
        }
        return res;
    }
}
