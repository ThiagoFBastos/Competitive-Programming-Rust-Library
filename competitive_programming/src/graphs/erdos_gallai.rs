use std::{cmp::Reverse, collections::VecDeque};

/**
 * @brief Check if is possible build a graph given a list of degrees
 * @param degrees The list of degrees
 */
pub fn erdos_gallai(degrees: &[i32]) -> bool {
    let mut deg = degrees.to_vec();

    deg.sort_by_key(|&d| Reverse(d));

    let n = deg.len();

    let mut left_sum = deg.iter().map(|d| *d as i64).sum::<i64>();
    let mut right_sum = 0;
    let mut cnt = 0;

    let mut st = VecDeque::new();

    let mut is_possible = left_sum % 2 == 0;

    for i in (1..=n).rev() {
        if !is_possible {
            break;
        }

        while let Some(&top) = st.front()
            && top >= i as i32
        {
            st.pop_front();
            right_sum -= top as i64;
            cnt += 1;
        }

        is_possible =
            is_possible && left_sum <= i as i64 * (i - 1) as i64 + right_sum + cnt * i as i64;

        st.push_front(deg[i - 1]);
        left_sum -= deg[i - 1] as i64;
        right_sum += deg[i - 1] as i64;
    }

    is_possible
}
