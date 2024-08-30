class Solution:
    def strStr(self, haystack: str, needle: str) -> int:
        prefixLen = self.calcPrefixLen(needle)

        # print(list(needle))
        # print(prefixLen)

        hlen = len(haystack)
        nlen = len(needle)

        h = 0
        n = 0

        matches = []

        while h < hlen:
            if needle[n] == haystack[h]:
                h += 1
                n += 1

                if n == nlen:
                    return h-n
                    matches.append(h - n)
                    n = prefixLen[n]
            else:
                n = prefixLen[n]
                if n < 0:
                    h += 1
                    n += 1
        # print(matches)
        return matches[0] if matches else -1

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

