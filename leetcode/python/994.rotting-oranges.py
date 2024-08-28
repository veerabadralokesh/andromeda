class Solution:
    def orangesRotting(self, grid: List[List[int]]) -> int:
        q = deque([])
        m, n = len(grid), len(grid[0])
        minutes = [[0] * n for _ in range(m)]
        freshOranges = 0
        for i in range(m):
            for j in range(n):
                if grid[i][j] == 1:
                    minutes[i][j] = 1000
                    freshOranges += 1
                elif grid[i][j] == 2:
                    q.append([i, j])
        
        time = 0
        dirs = [[0, 1], [1, 0], [-1, 0], [0, -1]]
        while q and freshOranges > 0:
            time += 1
            rotten_count = len(q)
            for _ in range(rotten_count):
                i, j = q.popleft()
                # print(i, j)
                for dij in dirs:
                    di = i + dij[0]
                    dj = j + dij[1]
                    if di >= m or dj >= n or di < 0 or dj < 0 or minutes[di][dj] != 1000:
                        continue
                    freshOranges -= 1
                    minutes[di][dj] = min(minutes[di][dj], time)
                    q.append([di, dj])

        # for row in minutes:
        #     print(row)
        # print(freshOranges)
        return -1 if freshOranges > 0 else time

