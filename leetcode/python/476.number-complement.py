class Solution:
    def findComplement(self, num: int) -> int:
        i = 31
        while (1 << i) & num == 0:
            i -= 1
        return num ^ ((1 << (i+1))-1)

