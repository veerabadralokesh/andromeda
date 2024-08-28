class Solution:
    def isPossibleToCutPath(self, grid: List[List[int]]) -> bool:
        m, n = len(grid)-1, len(grid[0])-1

        def pathExists(i: int, j: int) -> bool:
            if i == m and j == n:
                return True
            if i > m or j > n:
                return False
            if grid[i][j] == 0:
                return False
            grid[i][j] = 0
            return pathExists(i + 1, j) or pathExists(i, j+1)
        
        if not pathExists(0, 0):
            return True

        grid[0][0] = 1

        return not pathExists(0, 0)

