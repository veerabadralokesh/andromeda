class Solution:
    def shortestPalindrome(self, s: str) -> str:
        prefix = s[::-1]

        for i in range(len(prefix)):
            if s.startswith(prefix[i:]):
                return prefix[:i] + s
        
        return prefix + s

