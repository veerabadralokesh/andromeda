class Solution:
    def maximumRows(self, matrix: List[List[int]], numSelect: int) -> int:
        m, n = len(matrix), len(matrix[0])
        mat = [0] * m
        for i,row in enumerate(matrix):
            r = 0
            for rc in row:
                r = r * 2 + rc
            mat[i] = r

        print(mat)
        
        ans = 0
        def backtrack(rem: int, mask: int, start: int):
            if rem == 0:
                nonlocal ans
                count = sum([1 if r & mask == r else 0 for r in mat])
                ans = max(ans, count)
                return
            
            for i in range(start, n-rem+1):
                if mask & (1 << i) == 0:
                    backtrack(rem-1, mask | (1 << i), i+1)
        
        backtrack(numSelect, 0, 0)
        
        return ans

