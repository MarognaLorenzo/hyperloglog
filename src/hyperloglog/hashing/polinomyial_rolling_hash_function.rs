mod polinomyial_rolling_hash_function;
pub struct PolRolHF {
    p: u64,
    m: u64,
}

impl PolRolHF {
    pub fn new (_p: u64, _m:u64) -> PolRolHF {
        return PolRolHF{ p: _p, m: _m};
    }
}

