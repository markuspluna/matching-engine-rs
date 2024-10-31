//quantity.rs

use alloy::primitives::U256;
use std::ops::{AddAssign, SubAssign};

#[derive(Debug, Default, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
pub struct Qty(pub U256);

impl AddAssign for Qty {
    fn add_assign(&mut self, other: Qty) {
        self.0 += other.0;
    }
}

impl SubAssign for Qty {
    fn sub_assign(&mut self, other: Qty) {
        self.0 -= other.0;
    }
}

impl Qty {
    #[inline]
    pub fn value(&self) -> U256 {
        self.0
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0 == U256::ZERO
    }
}
