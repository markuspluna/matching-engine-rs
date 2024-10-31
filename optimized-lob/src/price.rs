// price.rs

use alloy::primitives::{I256, U256};

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Debug, Default)]
pub struct Price(pub I256);

impl Price {
    /// Returns the value of the price.
    #[inline]
    pub fn value(&self) -> I256 {
        self.0
    }

    /// Returns true if the price is a bid.
    #[inline]
    pub fn is_bid(&self) -> bool {
        self.0 > I256::ZERO
    }

    /// Returns the absolute value of the price.
    #[inline]
    pub fn absolute(&self) -> U256 {
        self.0.unsigned_abs()
    }

    /// Convert a u256 to a Price.
    #[inline]
    pub fn from_u256(price: U256, is_bid: bool) -> Self {
        let signed = I256::try_from(price).unwrap();
        Self(if is_bid { signed } else { -signed })
    }
}
