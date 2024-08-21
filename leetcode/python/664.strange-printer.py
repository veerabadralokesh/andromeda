class Solution:
    def strangePrinter(self, s: str) -> int:
        n = len(s)
        if n == 0:
            return 0

        dp = [[0] * n for _ in range(n)]

        return Solution.helper(s, 0, n - 1, dp)

    def helper(s: str, i: int, j: int, dp: List[List[int]]):
        if i > j:
            return 0
        if dp[i][j] == 0:
            dp[i][j] = Solution.helper(s, i + 1, j, dp) + 1

            for k in range(i + 1, j + 1):
                if s[i] == s[k]:
                    dp[i][j] = min(
                        dp[i][j],
                        Solution.helper(s, i, k - 1, dp)
                        + Solution.helper(s, k + 1, j, dp),
                    )

        return dp[i][j]

""" """

class Solution:
    def strangePrinter(self, s: str) -> int:
        n = len(s)
        if n == 0:
            return 0

        dp = [[n] * n for _ in range(n)]

        for i in range(n):
            dp[i][i] = 1

        for j in range(n):
            for i in range(j, -1, -1):
                for k in range(i, j):
                    dp[i][j] = min(
                        dp[i][j], dp[i][k] + dp[k + 1][j] - (1 if s[j] == s[k] else 0)
                    )

        return dp[0][n - 1]


