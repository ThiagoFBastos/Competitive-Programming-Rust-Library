/*
 * Author: Thiago Felipe Bastos da Silva
 * Created: 2026-01-24
 * Description: This function implements the Manacher Algorithm that finds the largest palindrome substring in a string
 */

/**
 * Find the largest palindrome substring size
 * @param s the target string
 * @return the size of the largest palindrome substring
 */
pub fn manacher(s: &String) -> usize {
    let n = s.len();
    let str = s.chars().collect::<Vec<_>>();

    let mut d1 = vec![1; n];
    let mut d2 = vec![0; n];

    let mut l = 0;
    let mut r = -1;
    let mut len = 0;

    for i in 0..n {

        if i as i32 <= r {
            d1[i] = d1[l + r as usize - i].min(r as usize + 1 - i);
            d2[i] = d2[l + r as usize + 1 - i].min(r as usize + 1 - i);
        }

        while i + d1[i] < n && i >= d1[i] && str[i - d1[i]] == str[i + d1[i]] {
            d1[i] += 1;
        }

        while i + d2[i] < n && i >= d2[i] + 1 && str[i - d2[i] - 1] == str[i + d2[i]] {
            d2[i] += 1;
        }

        if r < i as i32 + d2[i] as i32 - 1 {
            l = i - d2[i];
            r = i as i32 + d2[i] as i32 - 1;
        }

        if r < i as i32 + d1[i] as i32 - 1 {
            l = i + 1 - d1[i];
            r = i as i32 + d1[i] as i32 - 1;
        }

        
        len = len.max(r as usize + 1 - l);
    }

    len
}