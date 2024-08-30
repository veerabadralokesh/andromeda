class Solution:
    def longestPrefix(self, s: str) -> str:
        prefixLen = self.calcPrefixLen(s)
        # print(prefixLen)
        return s[:prefixLen[-1]]
    

    ## calculating KMP prefix arr
    def calcPrefixLen(self, pattern: str) -> List[int]:
        patternLen = len(pattern)
        arr = [0] * (patternLen + 1)
        arr[0] = -1

        prefixLen = 0
        i = 1

        while i < patternLen:
            if pattern[prefixLen] == pattern[i]:
                prefixLen += 1
                i += 1
                arr[i] = prefixLen
            elif prefixLen > 0:
                prefixLen = arr[prefixLen]
            else:
                i += 1
                arr[i] = 0
        
        return arr

