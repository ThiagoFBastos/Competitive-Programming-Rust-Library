
#[cfg(test)]
mod segment_tree_tests {
    use competitive_programming::data_structures::segment_tree::*;

    #[test]
    fn test_min_value_in_interval() {

        #[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
        struct Int32(i32);

        impl Constants for Int32 {
            fn initial() -> Self {
                return Int32(i32::MAX);
            }
        }

        let operation = |a: Int32, b: Int32| {
            Int32(a.0.min(b.0))
        };

        let values = [1, 2, 3, -1, 4, 5].iter().map(|value| Int32(*value)).collect::<Vec<_>>();

        let mut st = SegTree::new(&values, operation);

        assert_eq!(st.query(0, 2), Int32(1));
        assert_eq!(st.query(1, 3), Int32(-1));
        assert_eq!(st.query(3, 5), Int32(-1));
        assert_eq!(st.query(4, 5), Int32(4));
        assert_eq!(st.query(0, 5), Int32(-1));

        st.update(0, Int32(-100));

        assert_eq!(st.query(0, 3), Int32(-100));
        assert_eq!(st.query(1, 5), Int32(-1));
    }

    #[test]
    fn test_sum_values_in_interval() {
        #[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
        struct Int32(i32);

        impl Constants for Int32 {
            fn initial() -> Self {
                return Int32(0);
            }
        }

        let operation = |a: Int32, b: Int32| {
            Int32(a.0 + b.0)
        };

        let values = [9, 1, 4, 3, 5].iter().map(|value| Int32(*value)).collect::<Vec<_>>();

        let mut st = SegTree::new(&values, operation);

        assert_eq!(st.query(0, 1), Int32(10));
        assert_eq!(st.query(2, 3), Int32(7));
        assert_eq!(st.query(2, 4), Int32(12));

        st.update(2, Int32(-9));

        assert_eq!(st.query(0, 1), Int32(10));
        assert_eq!(st.query(2, 3), Int32(-6));
        assert_eq!(st.query(2, 4), Int32(-1));        
    }
}