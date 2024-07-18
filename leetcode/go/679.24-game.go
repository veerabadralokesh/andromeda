import "math"
func judgePoint24(cards []int) bool {
    nums := make([]float64, len(cards))
    for i, n := range cards {
        nums[i] = float64(n)
    }
    return dfs(nums)
}

func prod_div_sum_diff(a float64, b float64) []float64 {
    a_by_b := math.MaxFloat64
    b_by_a := math.MaxFloat64
    if a != 0.0 {
        b_by_a = b / a
    }
    if b != 0.0 {
        a_by_b = a / b
    }
    return []float64{a * b, a_by_b, b_by_a, a + b, a - b, b - a}
};

func abs(x float64) float64 {
    if x < 0 {
        return -x
    }
    return x
}

func dfs(nums []float64) bool {
    if len(nums) == 1 {
        return abs(nums[0] - 24.0) < 0.001
    }
    for i:=0; i < len(nums); i++ {
        for j:=i+1; j < len(nums); j++ {
            for _, n := range prod_div_sum_diff(nums[i], nums[j]) {
                if n == math.MaxFloat64 {
                    continue
                }
                new_nums := []float64{n}
                for k:=0; k < len(nums); k++ {
                    if k != i && k != j {
                        new_nums = append(new_nums, nums[k])
                    }
                }
                if dfs(new_nums) {
                    return true
                }
            }
        }
    }
    return false
}

