/*
 * Author: Thiago Felipe Bastos da Silva
 * Created: 2025-12-28
 * Description: The function below implements the KMP algorithm for a given string.
 */

 /**
  * KMP algorithm for strings
  * @param str the target string
  * @return an array containing the result of KMP algorithm
  */
pub fn kmp(str: &String) -> Vec<usize> {

   let chars = str.chars().collect::<Vec<_>>();

   let mut kmp = Vec::with_capacity(str.len());

   kmp.push(0);

   for i in 1..chars.len() {
      let mut k = kmp[i - 1];

      while k > 0 && chars[k] != chars[i] {
         k = kmp[k - 1];
      }

      if chars[k] == chars[i] {
         k += 1;
      }

      kmp.push(k);
   }

   kmp
}
