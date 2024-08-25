class Solution:
    def coinChange(self, coins: List[int], amount: int) -> int:
        dp = [float('inf')] * (amount + 1)
        coins.sort()
        dp[0] = 0
        for a in range(1, amount + 1):
            for coin in coins:
                if coin > a:
                    break
                dp[a] = min(dp[a], dp[a - coin] + 1)
        return -1 if dp[amount] == float('inf') else dp[amount]

