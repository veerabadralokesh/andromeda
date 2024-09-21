class Solution:
    def lexicalOrder(self, n: int) -> List[int]:
        ans = []
        def genNums(i: int):
            ans.append(i)
            for j in range(i*10, i*10+10):
                if j > n:
                    break
                genNums(j)
        for x in range(1, 10):
            if x > n:
                break
            genNums(x)
        return ans

