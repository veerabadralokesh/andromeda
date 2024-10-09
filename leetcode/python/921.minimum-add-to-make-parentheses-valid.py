class Solution:
    def minAddToMakeValid(self, s: str) -> int:
        openCount = 0
        closeCount = 0
        for c in s:
            if c == '(':
                openCount += 1
            elif openCount > 0:
                openCount -= 1
            else:
                closeCount += 1
        return openCount + closeCount

