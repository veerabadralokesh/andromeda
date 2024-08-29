class Solution:
    def removeStones(self, stones: List[List[int]]) -> int:
        n = len(stones)
        graph = [[] for _ in range(n)]

        for i in range(n):
            for j in range(i+1, n):
                if stones[i][0] == stones[j][0] or stones[i][1] == stones[j][1]:
                    graph[i].append(j)
                    graph[j].append(i)
        
        visited = set()

        def dfs(u: int):
            nonlocal visited
            for v in graph[u]:
                if v not in visited:
                    visited.add(v)
                    dfs(v)

        ans = n

        for u in range(n):
            if u not in visited:
                dfs(u)
                ans -= 1

        return ans

