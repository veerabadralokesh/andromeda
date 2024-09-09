impl Solution {
    pub fn result_grid(mut image: Vec<Vec<i32>>, threshold: i32) -> Vec<Vec<i32>> {
        let (m, n) = (image.len(), image[0].len());

        let mut sum = vec![vec![0; n]; m];
        let mut counts = vec![vec![0; n]; m];

        let is_valid_region = |i: usize, j: usize| -> bool {
            for x in i..i+3 {
                for y in j..j+3 {
                    if x > i && (image[x][y] - image[x-1][y]).abs() > threshold {
                        return false;
                    }
                    if y > j && (image[x][y] - image[x][y-1]).abs() > threshold {
                        return false;
                    }
                }
            }
            true
        };
        let mut subgrid_sum = 0;
        for i in 0..m-2 {
            for j in 0..n-2 {
                if is_valid_region(i, j) {
                    subgrid_sum = 0;
                    for x in i..i+3 {
                        for y in j..j+3 {
                            subgrid_sum += image[x][y];
                        }
                    }
                    subgrid_sum = subgrid_sum / 9;
                    for x in i..i+3 {
                        for y in j..j+3 {
                            sum[x][y] += subgrid_sum;
                            counts[x][y] += 1;
                        }
                    }
                }
            }
        }
        for i in 0..m {
            for j in 0..n {
                if counts[i][j] > 0 {
                    image[i][j] = sum[i][j] / counts[i][j];
                }
            }
        }
        image
    }
}

