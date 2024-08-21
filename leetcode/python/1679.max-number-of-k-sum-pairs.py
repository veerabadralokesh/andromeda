class Solution:
    def maxOperations(self, nums: List[int], k: int) -> int:
        counts = defaultdict(int)
        for n in nums:
            counts[n] += 1
        ans = 0
        for x in counts.keys():
            y = k-x
            if x == y:
                count = counts.get(x)//2
            else:
                count = min(counts.get(x, 0), counts.get(y, 0))
            if count > 0:
                ans += count
                counts[x] -= count
                counts[y] -= count
        return ans

