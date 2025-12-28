/*
 * Author: Thiago Felipe Bastos da Silva
 * Created: 2025-12-28
 * Description: Find \sum_{i=0}^{N-1} \left\lfloor \frac{A i + B}{M} \right\rfloor.
 */

/**
 * Sum integers inside a given interval
 * @param a the leftmost number inside the interval
 * @param b the rightmost number inside the interval
 * @return the sum of all integers between a and b
 */
fn linsum(a: i128, b: i128) -> i128 {
   return (a + b) * (b - a + 1) / 2;
}

/**
 * Find the \sum_{i=0}^{n-1} \left( (a i + b) \bmod m \right)
 */
fn sum(n: i128, m: i128, a: i128, b: i128) -> i128 {
   if a == 0 {
      return b * n;
   }

	let k = (a * (n - 1) + b) / m;
	let s = b * n + a * linsum(0, n - 1);
	let t = k * (n - 1) - ((a+(-b-1)%a)%a+m * linsum(1, k) - k * (b + 1) - sum(k + 1, a, m % a, (a+(-b-1)%a)%a)) / a;

	return s - m * t;
}

/**
 * Find \sum_{i=0}^{n-1} \left\lfloor \frac{a i + b}{m} \right\rfloor
 */
fn query(n: i128, m: i128, a: i128, b: i128) -> i128 {
   let s = b * n + a * linsum(0, n - 1) - sum(n, m, a, b);
   return s / m;
}
