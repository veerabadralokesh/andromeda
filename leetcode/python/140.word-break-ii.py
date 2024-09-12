class Solution:

    def wordBreak(self, s: str, wordDict: List[str]) -> List[str]:
        
        self.memo = {}
        ans = []
        self.helper(s, wordDict, ans, [])
        return ans


    def helper(self, s, wordDict, ans, curr_result):

        if s == '':
            ans.append(' '.join(curr_result))
            return
        
        for word in wordDict:
            if s.startswith(word):
                curr_result.append(word)
                self.helper(s[len(word):], wordDict, ans, curr_result)
                curr_result.pop()
        

