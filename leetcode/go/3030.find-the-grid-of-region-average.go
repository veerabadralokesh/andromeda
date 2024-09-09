func resultGrid(image [][]int, threshold int) [][]int {
     m, n := len(image), len(image[0])

    sum := make([][]int, m)
    counts := make([][]int, m)
    for i := range sum {
        sum[i] = make([]int, n)
        counts[i] = make([]int, n)
    }

    is_valid_region := func(i, j int) bool {
        for x := i; x < i+3; x++ {
            for y := j; y < j+3; y++ {
                if x > i && abs(image[x][y]-image[x-1][y]) > threshold {
                    return false
                }
                if y > j && abs(image[x][y]-image[x][y-1]) > threshold {
                    return false
                }
            }
        }
        return true
    }

    for i := 0; i < m-2; i++ {
        for j := 0; j < n-2; j++ {
            if is_valid_region(i, j) {
                subgrid_sum := 0
                for x := i; x < i+3; x++ {
                    for y := j; y < j+3; y++ {
                        subgrid_sum += image[x][y]
                    }
                }
                subgrid_sum /= 9
                for x := i; x < i+3; x++ {
                    for y := j; y < j+3; y++ {
                        sum[x][y] += subgrid_sum
                        counts[x][y] += 1
                    }
                }
            }
        }
    }

    for i := 0; i < m; i++ {
        for j := 0; j < n; j++ {
            if counts[i][j] > 0 {
                image[i][j] = sum[i][j] / counts[i][j]
            }
        }
    }

    return image
}

func abs(x int) int {
    if x < 0 {
        return -x
    }
    return x
}

