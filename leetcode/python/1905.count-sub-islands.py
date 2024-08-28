class Solution:
    def countSubIslands(self, grid1: List[List[int]], grid2: List[List[int]]) -> int:
        dirs = [[0, 1], [0, -1], [1, 0], [-1, 0]]
        m, n = len(grid1), len(grid1[0])
        def bfs(i: int, j: int) -> int:
            isSubIsland = 1 if grid1[i][j] == 1 else 0
            q = deque([[i, j]])
            grid2[i][j] = 2
            while q:
                x, y = q.popleft()
                for dxy in dirs:
                    dx = x + dxy[0]
                    dy = y + dxy[1]
                    if dx < 0 or dy < 0 or dx == m or dy == n or grid2[dx][dy] != 1:
                        continue
                    if grid1[dx][dy] == 0:
                        isSubIsland = 0
                    grid2[dx][dy] = 2
                    q.append([dx, dy])
            return isSubIsland
        ans = 0
        for i in range(m):
            for j in range(n):
                if grid2[i][j] == 1:
                    ans += bfs(i, j)
        return ans

