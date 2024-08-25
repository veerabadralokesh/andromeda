class Solution:
    def numTilings(self, n: int) -> int:
        dp = [0] * max(4, n+1)
        dp[1], dp[2], dp[3] = 1, 2, 5
        mod = 1_000_000_007
        for i in range(4, n+1):
            dp[i] = (2 * dp[i-1] + dp[i-3]) % mod
            # while dp[i] >= mod:
            #     dp[i] -= mod
        return dp[n]

