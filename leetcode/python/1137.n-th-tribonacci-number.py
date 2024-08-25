class Solution:
    def tribonacci(self, n: int) -> int:
        track = [0, 1, 1]
        if n < 3:
            return track[n]
        i = 0
        for j in range(3, n):
            track[i] = sum(track)
            i = 0 if i == 2 else i + 1
        return sum(track)

