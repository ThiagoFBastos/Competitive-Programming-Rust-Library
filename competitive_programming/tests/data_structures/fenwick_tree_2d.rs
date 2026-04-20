#[cfg(test)]
mod tests {
    use competitive_programming::data_structures::FenwickTree2D;

    #[test]
    fn test_sum_query() {
        const R: usize = 5;
        const C: usize = 5;

        let mut ft = FenwickTree2D::<i32>::new(R, C);

        for x0 in 1..=R {
            for x1 in x0..=R {
                for y0 in 1..=C {
                    for y1 in y0..=C {
                        let sum = ft.calculate(x0, y0, x1, y1);
                        assert_eq!(sum, 0);
                    }
                }
            }
        }

        for i in 1..=R {
            for j in 1..=C {
                ft.update(i, j, 1); // TODO add random numbers
            }
        }

        for x0 in 1..=R {
            for x1 in x0..=R {
                for y0 in 1..=C {
                    for y1 in y0..=C {
                        let sum = ft.calculate(x0, y0, x1, y1);
                        let rectangle_sum = (x1 - x0 + 1) * (y1 - y0 + 1);
                        assert_eq!(sum, rectangle_sum as i32);
                    }
                }
            }
        }
    }
}
