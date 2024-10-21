class Solution:
    def maxUniqueSplit(self, s: str) -> int:
        ans = 0
        def backtrack(i: int, uniq) -> int:
            if i == len(s):
                nonlocal ans
                ans = max(ans, len(uniq))
                return
            for j in range(i+1, len(s)+1):
                if s[i:j] in uniq:
                    continue
                uniq.add(s[i:j])
                backtrack(j, uniq)
                uniq.remove(s[i:j])
        uniq = set()
        backtrack(0, uniq)
        return ans

