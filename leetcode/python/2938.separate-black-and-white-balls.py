class Solution:
    def minimumSteps(self, s: str) -> int:
        ones, ans = 0, 0
        for c in s:
            if c == '1':
                ones += 1
            else:
                ans += ones
        return ans

