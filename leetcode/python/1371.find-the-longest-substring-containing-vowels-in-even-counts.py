class Solution:
    def findTheLongestSubstring(self, s: str) -> int:
        mask = 0
        previous_mask = {0: -1}
        ans = 0
        for i,c in enumerate(s):
            if c in "aeiou":
                mask = mask ^ (1 << (ord(c) - ord('a')))
            if mask in previous_mask:
                ans = max(ans, i - previous_mask[mask])
            else:
                previous_mask[mask] = i
        return ans

