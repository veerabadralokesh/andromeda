class Solution:
    def change(self, amount: int, coins: List[int]) -> int:
        dp = [0] * (amount + 1)
        dp[0] = 1
        for coin in coins:
            for a in range(coin, amount + 1):
                dp[a] += dp[a-coin]
        return dp[amount]

""" """

class Solution:
    def change(self, amount: int, coins: List[int]) -> int:
        @cache
        def countCombos(i: int, a: int) -> int:
            if i == len(coins):
                return 1 if a == 0 else 0
            take = 0
            if coins[i] <= a:
                take = countCombos(i, a-coins[i])
            skip = countCombos(i + 1, a)
            return take + skip
        return countCombos(0, amount)

