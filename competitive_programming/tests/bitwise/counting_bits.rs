#[cfg(test)]
mod tests {
    use competitive_programming::bitwise::counting_bits;

    #[test]
    fn test_counting_bits() {
        assert_eq!(counting_bits(1), 1);
        assert_eq!(counting_bits(2), 2);
        assert_eq!(counting_bits(3), 4);
        assert_eq!(counting_bits(4), 5);
        assert_eq!(counting_bits(5), 7);
        assert_eq!(counting_bits(6), 9);
        assert_eq!(counting_bits(7), 12);
        assert_eq!(counting_bits(8), 13);
        assert_eq!(counting_bits(9), 15);
        assert_eq!(counting_bits(10), 17);
        assert_eq!(counting_bits(303021765044187), 7250167017244884);
        assert_eq!(counting_bits(390977392667778), 9400236781929604);
        assert_eq!(counting_bits(670904313808571), 16399391652009372);
        assert_eq!(counting_bits(704275111916256), 17252936460583174);
        assert_eq!(counting_bits(799807335176164), 19646099779305746);
        assert_eq!(counting_bits(852012055430877), 21005540865890493);
        assert_eq!(counting_bits(901305628563213), 22232965316761961);
        assert_eq!(counting_bits(917427461591619), 22650112032503064);
        assert_eq!(counting_bits(981787468874797), 24322274256024775);
        assert_eq!(counting_bits(1000000000000000), 24784747400675348);
        assert_eq!(counting_bits(989898989898989), 24531282955144033);
    }
}
