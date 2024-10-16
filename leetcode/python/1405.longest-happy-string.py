class Solution:
    def longestDiverseString(self, a: int, b: int, c: int, A = 'a', B = 'b', C = 'c') -> str:
        if a < b:
            return self.longestDiverseString(b, a, c, B, A, C)
        if b < c:
            return self.longestDiverseString(a, c, b, A, C, B)
        if b == 0:
            return A * min(a, 2)
        
        aCount = min(a, 2)
        bCount = 1 if (a - aCount >= b) else 0
        return A * aCount + B * bCount + self.longestDiverseString(a - aCount, b - bCount, c, A, B, C)

