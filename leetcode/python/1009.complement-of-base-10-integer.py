class Solution:
    def bitwiseComplement(self, n: int) -> int:
        i = 31
        while i > 0 and (1 << i) & n == 0:
            i -= 1
        return n ^ ((1 << (i+1))-1)

