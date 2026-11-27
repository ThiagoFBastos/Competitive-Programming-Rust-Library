use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ModNumber<const MOD: i64> {
    number: i64, // number (mod MOD)
}

impl<const MOD: i64> ModNumber<MOD> {
    pub const MODULO: i64 = MOD;

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

        Self { number: initial }
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
        let mut number = *self;

        while n > 0 {
            if n & 1 == 1 {
                result *= number;
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
            return ModNumber::new(
                ((self.number as i128 * rhs.number as i128) % MOD as i128) as i64,
            );
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructor_with_negative_one() {
        const MOD: i64 = 1_000_000_007;

        let number = ModNumber::<MOD>::new(-1);

        assert_eq!(number.value(), MOD - 1);
        assert_eq!(ModNumber::<MOD>::MODULO, MOD);
    }

    #[test]
    fn test_constructor_with_small_negative() {
        const MOD: i64 = 1_000_000_007;

        let number = ModNumber::<MOD>::new(-MOD);

        assert_eq!(number.value(), 0);
    }

    #[test]
    fn test_constructor_with_large_negative() {
        const MOD: i64 = 1_000_000_007;

        let number = ModNumber::<MOD>::new(-2 * MOD - 1);

        assert_eq!(number.value(), MOD - 1);
    }

    #[test]
    fn test_constructor_with_small_positive() {
        const MOD: i64 = 1_000_000_007;

        let number = ModNumber::<MOD>::new(2 * MOD - 1);

        assert_eq!(number.value(), MOD - 1);
    }

    #[test]
    fn test_constructor_with_large_positive() {
        const MOD: i64 = 1_000_000_007;

        let number = ModNumber::<MOD>::new(2 * MOD);

        assert_eq!(number.value(), 0);
    }

    #[test]
    fn test_add_numbers() {
        const MOD: i64 = 1_000_000_007;

        let num1 = ModNumber::<MOD>::new(MOD - 1);
        let num2 = ModNumber::<MOD>::new(MOD - 1);

        let result = num1 + num2;

        assert_eq!(result.value(), MOD - 2);
    }

    #[test]
    fn test_subtract_numbers() {
        const MOD: i64 = 1_000_000_007;

        let num1 = ModNumber::<MOD>::new(0);
        let num2 = ModNumber::<MOD>::new(1);

        let result = num1 - num2;

        assert_eq!(result.value(), MOD - 1);
    }

    #[test]
    fn test_multiply_numbers() {
        const MOD: i64 = 1_000_000_007;

        let num1 = ModNumber::<MOD>::new(191919181);
        let num2 = ModNumber::<MOD>::new(81837373);

        let result = num1 * num2;

        assert_eq!(result.value(), 491_408_386);
    }

    #[test]
    fn test_divide_numbers() {
        const MOD: i64 = 1_000_000_007;

        let num1 = ModNumber::<MOD>::new(4);
        let num2 = ModNumber::<MOD>::new(2);

        let result = num1 / num2;

        assert_eq!(result.value(), 2);
    }

    #[test]
    fn test_add_assign_numbers() {
        const MOD: i64 = 1_000_000_007;

        let mut result = ModNumber::<MOD>::new(MOD - 1);
        result += ModNumber::<MOD>::new(MOD - 1);

        assert_eq!(result.value(), MOD - 2);
    }

    #[test]
    fn test_subtract_assign_numbers() {
        const MOD: i64 = 1_000_000_007;

        let mut result = ModNumber::<MOD>::new(0);
        result -= ModNumber::<MOD>::new(1);

        assert_eq!(result.value(), MOD - 1);
    }

    #[test]
    fn test_multiply_assign_numbers() {
        const MOD: i64 = 1_000_000_007;

        let mut result = ModNumber::<MOD>::new(191919181);
        result *= ModNumber::<MOD>::new(81837373);

        assert_eq!(result.value(), 491_408_386);
    }

    #[test]
    fn test_divide_assign_numbers() {
        const MOD: i64 = 1_000_000_007;

        let mut result = ModNumber::<MOD>::new(4);
        result /= ModNumber::<MOD>::new(2);

        assert_eq!(result.value(), 2);
    }

    #[test]
    fn test_raise_to_the_power() {
        const MOD: i64 = 1_000_000_007;

        let num = ModNumber::<MOD>::new(2);

        let result = num.pow(60);

        assert_eq!(result.value(), 536396504);
    }

    #[test]
    fn test_negative_the_number() {
        const MOD: i64 = 1_000_000_007;

        let num = ModNumber::<MOD>::new(1);

        let result = -num;

        assert_eq!(result.value(), MOD - 1);
    }

    #[test]
    fn test_multiply_large_numbers() {
        const MOD: i64 = 1_000_000_007;

        let num1 = ModNumber::<MOD>::new(939399393933939393);
        let num2 = ModNumber::<MOD>::new(848484292929928282);

        let result = num1 * num2;

        assert_eq!(result.value(), 258938338);
    }

    #[test]
    fn test_compare_equal_numbers() {
        const MOD: i64 = 1_000_000_007;

        let num1 = ModNumber::<MOD>::new(1);
        let num2 = ModNumber::<MOD>::new(MOD + 1);

        assert!(num1 == num2);
    }

    #[test]
    fn test_compare_different_numbers() {
        const MOD: i64 = 1_000_000_007;

        let num1 = ModNumber::<MOD>::new(1);
        let num2 = ModNumber::<MOD>::new(2);

        assert!(num1 != num2);
    }
}
