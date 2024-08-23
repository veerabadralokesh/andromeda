class Solution:
    def equalPairs(self, grid: List[List[int]]) -> int:
        hashmap = defaultdict(int)

        for row in grid:
            hashmap["-".join([str(n) for n in row])] += 1
        
        pairs = 0
        for j in range(len(grid)):
            col = []
            for i in range(len(grid)):
                col.append(str(grid[i][j]))
            
            pairs += hashmap["-".join(col)]
        
        return pairs

