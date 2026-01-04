/*
 * Author: Thiago Felipe Bastos da Silva
 * Created: 2026-01-04
 * Description: Structure that encapsulates modulo operations
 */

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, PartialEq)]
pub struct ModNumber<const MOD: i64> {
    number: i64 // number (mod MOD)
}

impl<const MOD: i64> ModNumber<MOD> {

    /**
     * Create a new instance of ModNumber
     * @param initial the value that will be stored
     */
    pub fn new(mut initial: i64) -> Self {
        assert!(MOD >= 2);

        if initial >= 2 * MOD {
            initial %= MOD;
        } else if initial < -MOD {
            initial = (initial % MOD + MOD) % MOD;
        } else if initial >= MOD {
            initial -= MOD;
        } else if initial < 0 {
            initial += MOD;
        }

        Self {
            number: initial
        }
    }

    /**
     * Return the value of number (mod MOD)
     */
    pub fn value(&self) -> i64 {
        self.number
    }

    /**
     *  Return the stored number raised to the power of n
     */  
    pub fn pow(&self, mut n: u64) -> Self {
        let mut result = Self::new(1);
        let mut number = self.clone();

        while n > 0 {

            if n & 1 == 1 {
            result = result * number;
            }

            number = number * number;

            n >>= 1;
        }

        result
    }
}

// operator +
impl<const MOD: i64> Add for ModNumber<MOD> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self.number + rhs.number;

        if result >= MOD {
            result -= MOD;
        }

        ModNumber::new(result)
    }
}

// operator -
impl<const MOD: i64> Sub for ModNumber<MOD> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = self.number - rhs.number;

        if result < 0 {
            result += MOD;
        }

        ModNumber::new(result)
    }
}

// operator *  
impl<const MOD: i64> Mul for ModNumber<MOD> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {

        let result = self.number.checked_mul(rhs.number);

        if result.is_none() {
            return ModNumber::new(((self.number as i128 * rhs.number as i128) % MOD as i128) as i64);
        }

        ModNumber::new(result.unwrap() % MOD)
    }
}

// operator /   
impl<const MOD: i64> Div for ModNumber<MOD> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.pow(MOD as u64 - 2)
    }
}

// operator -val
impl<const MOD: i64> Neg for ModNumber<MOD> {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.number = MOD - self.number;

        if self.number >= MOD {
            self.number -= MOD;
        }

        self
    }
}

// operator +=
impl<const MOD: i64> AddAssign for ModNumber<MOD> {

    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

// operator -=
impl<const MOD: i64> SubAssign for ModNumber<MOD> {   

    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

// operator *=
impl<const MOD: i64> MulAssign for ModNumber<MOD> {

    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

// operator /=
impl<const MOD: i64> DivAssign for ModNumber<MOD> {

    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}
