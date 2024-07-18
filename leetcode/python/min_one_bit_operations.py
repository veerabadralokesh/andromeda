import math

class Solution:
    def minimumOneBitOperations(self, n: int) -> int:
        p = int(math.log2(n)) + 1
        px = (n ^ (2 ** p - 1)) + (2 ** (p - 1))
        print(n, px)
        return 5


if __name__ == "__main__":
    sol = Solution()
    for i in range(1, 17):
        sol.minimumOneBitOperations(i)
    