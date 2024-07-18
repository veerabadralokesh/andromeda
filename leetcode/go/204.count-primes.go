import (
    "math"
)
func countPrimes(n int) int {
    if n < 3 {
        return 0;
    }
    vec := make([]bool, n);
    for i,_ := range vec {vec[i] = true}
    vec[0] = false
    vec[1] = false
    sqrtn := int(math.Sqrt(float64(n)))
    i := 2
    for i <= sqrtn {
        if vec[i] {
            for j:=2 * i; j < n; j += i {
                vec[j] = false
            }
        }
        i++
    }
    count := 0
    for _, b := range vec {
        if b {
            count++
        }
    }
    return count
}

