pub trait Hashing {
    fn hash(&self, input: &String) -> u128;
}
pub struct PolRolHF{
    p: u128,
    m: u128,
    salt: u128,
}

impl PolRolHF {
    pub fn new(p: u128, m: u128, salt: u128) -> PolRolHF {
        PolRolHF {p, m, salt}
    }
}

impl Hashing for PolRolHF {
    fn hash(&self, input: &String) -> u128 {
        let word_as_bytes = input.as_bytes();
        let mut res: u128 = self.salt;

        let mut powering = 1;

        for i in 0..input.len() {
            let char_val: &u8 = word_as_bytes.get(i).unwrap();
            res = (res + *char_val as u128 * powering) % self.m;
            powering = (powering * self.p) % self.m;
        }
        return res;
    }
}
