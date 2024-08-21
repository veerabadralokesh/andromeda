class Solution:
    def uniqueOccurrences(self, arr: List[int]) -> bool:
        offset = 1000
        found = [0] * 2001
        count = 0
        for n in arr:
            if found[n] == 0:
                count += 1
            found[n] += 1
        return len(set(found)) == count + 1

