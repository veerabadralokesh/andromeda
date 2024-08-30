class Solution:
    def countPrefixSuffixPairs(self, words: List[str]) -> int:
        def isPrefixSuffix(wordi: int, wordj: int) -> int:
            return len(wordj) >= len(wordi) and wordj.startswith(wordi) and wordj.endswith(wordi)
        ans = 0
        for i in range(len(words)-1):
            for j in range(i+1, len(words)):
                ans += isPrefixSuffix(words[i], words[j])
        return ans

