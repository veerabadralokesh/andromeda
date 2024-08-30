class Solution:
    def stringMatching(self, words: List[str]) -> List[str]:
        ans = []
        for a in words:
            for b in words:
                if len(a) < len(b) and a in b:
                    ans.append(a)
                    break
        return ans

