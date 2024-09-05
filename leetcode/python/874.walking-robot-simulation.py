class Solution:
    def robotSim(self, commands: List[int], obstacles: List[List[int]]) -> int:
        dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)]
        ans = 0

        dir = 0
        x = 0
        y = 0

        obstacles = set([(i, j) for i, j in obstacles])

        for c in commands:
            if c == -1:
                dir = (dir + 1) % 4
            elif c == -2:
                dir = (dir + 3) % 4
            else:
                for _ in range(c):
                    if (x + dirs[dir][0], y + dirs[dir][1]) in obstacles:
                        break
                    x += dirs[dir][0]
                    y += dirs[dir][1]
                ans = max(ans, x * x + y * y)
        return ans

