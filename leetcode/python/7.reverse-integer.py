class Solution:
    def reverse(self, x: int) -> int:
        ans = 0
        sign = 1
        if x < 0:
            sign = -1
            x = -x
        while x > 0:
            ans = ans * 10 + (x % 10)
            x //= 10
        ans = ans * sign
        if ans >= pow(2, 31) or ans < -pow(2, 31):
            return 0
        return ans
