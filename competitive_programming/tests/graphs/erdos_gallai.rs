#[cfg(test)]
mod erdos_gallai_tests {
    use competitive_programming::graphs::erdos_gallai;

    #[test]
    fn empty_graph() {
        assert!(erdos_gallai(&[]));
    }

    #[test]
    fn single_vertex_degree_zero() {
        assert!(erdos_gallai(&[0]));
    }

    #[test]
    fn single_vertex_invalid_degree() {
        assert!(!erdos_gallai(&[1]));
    }

    #[test]
    fn two_vertices_connected() {
        assert!(erdos_gallai(&[1, 1]));
    }

    #[test]
    fn two_vertices_invalid() {
        assert!(!erdos_gallai(&[1, 0]));
    }

    #[test]
    fn odd_sum_must_fail() {
        assert!(!erdos_gallai(&[3, 3, 1]));
    }

    #[test]
    fn another_odd_sum_case() {
        assert!(!erdos_gallai(&[2, 2, 1, 1, 1]));
    }

    #[test]
    fn complete_graph() {
        assert!(erdos_gallai(&[4, 4, 4, 4, 4]));
    }

    #[test]
    fn path_graph() {
        assert!(erdos_gallai(&[2, 2, 2, 1, 1]));
    }

    #[test]
    fn cycle_graph() {
        assert!(erdos_gallai(&[2, 2, 2, 2]));
    }

    #[test]
    fn star_graph() {
        assert!(erdos_gallai(&[4, 1, 1, 1, 1]));
    }

    #[test]
    fn regular_graph() {
        assert!(erdos_gallai(&[3, 3, 3, 3]));
    }

    #[test]
    fn degree_too_large() {
        assert!(!erdos_gallai(&[5, 1, 1, 1, 1]));
    }

    #[test]
    fn violates_erdos_gallai_inequality() {
        assert!(!erdos_gallai(&[4, 4, 1, 1, 1, 1]));
    }

    #[test]
    fn another_invalid_sequence() {
        assert!(!erdos_gallai(&[3, 3, 3, 1]));
    }

    #[test]
    fn all_zeros() {
        assert!(erdos_gallai(&[0, 0, 0, 0]));
    }

    #[test]
    fn mixed_with_zeros_valid() {
        assert!(erdos_gallai(&[2, 2, 1, 1, 0]));
    }

    #[test]
    fn mixed_with_zeros_invalid() {
        assert!(!erdos_gallai(&[3, 1, 1, 0]));
    }

    #[test]
    fn unsorted_valid_sequence() {
        assert!(erdos_gallai(&[1, 4, 1, 1, 1]));
    }

    #[test]
    fn unsorted_invalid_sequence() {
        assert!(!erdos_gallai(&[1, 1, 4, 4, 1, 1]));
    }

    #[test]
    fn large_complete_graph() {
        const N: usize = 1000;

        let degrees = vec![(N - 1) as i32; N];

        assert!(erdos_gallai(&degrees));
    }

    #[test]
    fn large_star_graph() {
        const N: usize = 1000;

        let mut degrees = vec![1; N];
        degrees[0] = (N - 1) as i32;

        assert!(erdos_gallai(&degrees));
    }

    #[test]
    fn large_invalid_graph() {
        const N: usize = 1000;

        let mut degrees = vec![(N - 1) as i32; N];
        degrees[N - 1] = 0;

        assert!(!erdos_gallai(&degrees));
    }

    #[test]
    fn tight_equality_case() {
        assert!(erdos_gallai(&[3, 3, 2, 2, 2]));
    }

    #[test]
    fn near_boundary_invalid_case() {
        assert!(!erdos_gallai(&[4, 4, 2, 2, 1]));
    }

    #[test]
    fn many_zeros_one_positive() {
        assert!(!erdos_gallai(&[1, 0, 0, 0, 0]));
    }

    #[test]
    fn maximum_valid_for_small_n() {
        assert!(erdos_gallai(&[2, 2, 2]));
    }

    #[test]
    fn impossible_small_case() {
        assert!(!erdos_gallai(&[2, 2, 1]));
    }
}
