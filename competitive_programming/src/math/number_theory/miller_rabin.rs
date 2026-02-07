fn bin_exp(n: i128, p: i128, m: i128) -> i128 {
   if p == 0 {
      return 1;
   }

   let mut res = bin_exp(n, p >> 1, m);

   res = res * res % m;

   if p & 1 == 1 {
      res = res * n % m;
   }

   return res;
}

pub fn is_prime(n: i64) -> bool {
   let primes: [i64; 9] = [2, 3, 5, 7, 11, 13, 17, 19, 23];

   for &p in primes.iter() {
      if n % p == 0 {
         return n == p;
      }
   }

   if n < *primes.last().unwrap() {
      return false;
   }

   let mut t = n - 1;
   let mut s = 0;

   while t & 1 == 0 {
      t >>= 1;
      s += 1;
   }

   for &p in primes.iter() {
      let mut pow = bin_exp(p as i128, t as i128, n as i128);

      if pow == 1 {
         continue;
      }      

      let mut ok = false;

      for _ in 0..s {
         if pow == n as i128 - 1 {
            ok = true;
            break;
         }

         pow = pow * pow % n as i128;
      }

      if !ok {
         return false;
      }
   }

   return true;
}