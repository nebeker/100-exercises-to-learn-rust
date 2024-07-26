// TODO: implement the necessary traits to make the test compile and pass.
//  You *can't* modify the test.

use std::ops::Add;

//3rd printing  test results needs Debug
//5th Clone for Copy
#[derive(Debug, Clone)]
pub struct WrappingU32 {
    value: u32,
}

impl WrappingU32 {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

//first Add
impl Add for WrappingU32 {
    type Output = WrappingU32;

    fn add(self, rhs: Self) -> Self::Output {
        WrappingU32 {
            value: self.value + rhs.value,
        }
    }
}

//4th coy for multiple additions
impl Copy for WrappingU32 {}

//2nd assert_eq needs PartialEq
impl PartialEq for WrappingU32 {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ops() {
        let x = WrappingU32::new(42);
        let y = WrappingU32::new(31);
        let z = WrappingU32::new(u32::MAX);
        assert_eq!(x + y + y + z, WrappingU32::new(103));
    }
}
