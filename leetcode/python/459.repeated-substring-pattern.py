class Solution:
    def repeatedSubstringPattern(self, s: str) -> bool:
        prefixLen = self.calcPrefixLen(s)

        # print(prefixLen)

        repLen = len(s) - prefixLen[-1]

        return len(s) % repLen == 0 and repLen < len(s)
        
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

