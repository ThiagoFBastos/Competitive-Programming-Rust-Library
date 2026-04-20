#[cfg(test)]
mod tests {
    use competitive_programming::data_structures::FenwickTree2D;

    #[test]
    fn test_sum_query() {
        const R: i32 = 5;
        const C: i32 = 5;

        let mut ft = FenwickTree2D::<i32>::new(R as usize, C as usize);

        for x0 in 0..R {
            for x1 in x0..R {
                for y0 in 0..C {
                    for y1 in y0..C {
                        let sum = ft.calculate(x0, y0, x1, y1);
                        assert_eq!(sum, 0);
                    }
                }
            }
        }

        for i in 0..R {
            for j in 0..C {
                ft.update(i, j, 1); // TODO add random numbers
            }
        }

        for x0 in 0..R {
            for x1 in x0..R {
                for y0 in 0..C {
                    for y1 in y0..C {
                        let sum = ft.calculate(x0, y0, x1, y1);
                        assert_eq!(sum, (x1 - x0 + 1) * (y1 - y0 + 1))
                    }
                }
            }
        }
    }
}
