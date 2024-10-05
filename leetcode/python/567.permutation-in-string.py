class Solution:
    def checkInclusion(self, s1: str, s2: str) -> bool:
        if len(s1) > len(s2):
            return False
        s1counts = [0] * 26
        for c in s1:
            s1counts[ord(c)-ord('a')] += 1
        s2counts = [0] * 26
        l = len(s1)

        for i in range(l):
            s2counts[ord(s2[i])-ord('a')] += 1
        
        for i in range(l, len(s2)):
            if s1counts == s2counts:
                return True
            s2counts[ord(s2[i])-ord('a')] += 1
            s2counts[ord(s2[i-l])-ord('a')] -= 1
        
        if s1counts == s2counts:
            return True
        
        return False

