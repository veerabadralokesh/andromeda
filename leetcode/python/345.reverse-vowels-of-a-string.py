class Solution:
    def reverseVowels(self, s: str) -> str:
        sarr = list(s)
        indices = []
        for i, c in enumerate(list(s.lower())):
            if c == 'e' or c == 'a' or c == 'i' or c == 'o' or c == 'u':
                indices.append(i)
        left, right = 0, len(indices) - 1
        while left < right:
            i, j = indices[left], indices[right]
            sarr[i], sarr[j] = sarr[j], sarr[i]
            left += 1
            right -= 1
        return "".join(sarr)

