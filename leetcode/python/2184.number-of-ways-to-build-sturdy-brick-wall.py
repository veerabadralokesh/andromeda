## Aru's code
def buildWallAru(height: int, width: int, bricks: list[int]) -> int:
    kMod = 1_000_000_007

    wall = [[0] * width for _ in range(height)]
    visited = set()
    ans = 0
    memo = {}

    def dfs(ht, wd, visited, wall):
        nonlocal ans

        if wd > width or ht > height: # if height and width is exceeded
            return
        
        if ht == height-1 and wd == width and wall[0][0]: # if reached complete height and width and there is brick in the wall
            temp = tuple(tuple(i) for i in wall)
            if temp not in visited:
                visited.add(temp)
                ans += 1
            return

        if wd == width: # if reached width then increment height, start with width=0
            ht += 1
            wd = 0

        for i in range(len(bricks)):
            if ht >= height:
                continue

            if (
                ht >= 1 and 
                wd+bricks[i] != width and 
                sum(wall[ht][:wd+1]) + bricks[i] == sum(wall[ht-1][:wd+bricks[i]])
            ):
                continue
            
            wall[ht][wd] = bricks[i]
            dfs(ht, wd+bricks[i], visited, wall)
            wall[ht][wd] = 0

    dfs(0, 0,visited, wall)

    return ans


def buildWall(height: int, width: int, bricks: list[int]) -> int:
    kMod = 1_000_000_007

    rows = []

    def buildRows(width, row):
        for brick in bricks:
            if brick == width:
                rows.append(row)
            elif brick < width:
                newWidth = width - brick
                buildRows(newWidth, row | 2 << newWidth)

    buildRows(width, 0)

    n = len(rows)
    dp = [1] * n

    graph = [[] for _ in range(n)]

    for i in range(n):
        for j in range(n):
            if rows[i] & rows[j] == 0:
                graph[i].append(j)
    
    for _ in range(1, height):
        new_dp = [0] * n
        for u in range(n):
            for v in graph[u]:
                new_dp[u] = (new_dp[u] + dp[v]) % kMod
        dp = new_dp

    return sum(dp) % kMod


if __name__ == "__main__":

    height=2
    width=3
    bricks=[1,2]
    assert buildWall(height, width, bricks) == 2


    height = 76
    width = 9
    bricks = [6,3,5,1,9]

    x = buildWall(height, width, bricks)
    print(x)
    assert x == 201495766

