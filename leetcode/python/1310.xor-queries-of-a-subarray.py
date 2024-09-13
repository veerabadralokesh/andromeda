class Solution:
    def xorQueries(self, arr: List[int], queries: List[List[int]]) -> List[int]:
        xors = [0] * (len(arr) + 1)
        for i in range(len(arr)):
            xors[i+1] = xors[i] ^ arr[i]
        
        ans = [0] * len(queries)

        for i,q in enumerate(queries):
            ans[i] = xors[q[1] + 1] ^ xors[q[0]]
        return ans

