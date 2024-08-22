class Solution:
    def largestAltitude(self, gain: List[int]) -> int:
        maxAlt, alt = 0, 0
        for g in gain:
            alt += g
            maxAlt = max(maxAlt, alt)
        return maxAlt

