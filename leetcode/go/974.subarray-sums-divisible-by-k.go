func subarraysDivByK(nums []int, k int) int {
    var counts []int
    for i:=0; i < k; i++ {
        counts = append(counts, 0)
    }
    counts[0] = 1
    ans := 0
    psum := 0
    for _,n := range nums {
        psum = (psum + (n % k + k)) % k
        ans += counts[psum]
        counts[psum] += 1
    }
    return ans
}

/* */

func mod(a int, k int) int {
    return (a % k + k) % k
}

func subarraysDivByK(nums []int, k int) int {
    sum := 0
    dp := make([]int, k)

    ans := 0
    for i := 0; i < len(nums); i++ {
        sum += nums[i]
        ans += dp[mod(sum, k)]
        if sum % k == 0 {
            ans++
        }

        dp[mod(sum, k)]++
    }

    return ans
}

/* */

func mod(n int, k int) int {
    return (n % k + k) % k
}
func subarraysDivByK(nums []int, k int) int {
    counts := make([]int, k)
    counts[0] = 1
    ans := 0
    psum := 0
    for _,n := range nums {
        psum += n
        psum = mod(psum, k)
        ans += counts[psum]
        counts[psum] += 1
    }
    return ans
}

