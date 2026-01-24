
#[cfg(test)]
mod segment_tree_tests {
    use competitive_programming::data_structures::recursive_segment_tree::*;

    #[test]
    fn test_min_value_in_interval() {

        let operation = |a: i32, b: i32| {
            a.min(b)
        };

        let values = [1, 2, 3, -1, 4, 5];

        let mut st = SegTree::new(&values, operation);

        assert_eq!(st.query(0, 2), 1);
        assert_eq!(st.query(1, 3), -1);
        assert_eq!(st.query(3, 5), -1);
        assert_eq!(st.query(4, 5), 4);
        assert_eq!(st.query(0, 5), -1);

        st.update(0, -100);

        assert_eq!(st.query(0, 3), -100);
        assert_eq!(st.query(1, 5), -1);
    }

    #[test]
    fn test_sum_values_in_interval() {

        let operation = |a, b| {
            a + b
        };

        let values = [9, 1, 4, 3, 5];

        let mut st = SegTree::new(&values, operation);

        assert_eq!(st.query(0, 1), 10);
        assert_eq!(st.query(2, 3), 7);
        assert_eq!(st.query(2, 4), 12);

        st.update(2, -9);

        assert_eq!(st.query(0, 1), 10);
        assert_eq!(st.query(2, 3), -6);
        assert_eq!(st.query(2, 4), -1);        
    }
}