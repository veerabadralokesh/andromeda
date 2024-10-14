class Solution:
    def smallestRange(self, nums: List[List[int]]) -> List[int]:
        n = len(nums)
        minHeap = []
        mx = nums[0][0]
        mn = mx
        for i, l in enumerate(nums):
            minHeap.append((l[0], i, 0))
            mx = max(mx, l[0])
            mn = min(mn, l[0])
        
        heapq.heapify(minHeap)
        
        maxRange = mx
        minRange = mn

        while len(minHeap) == n:
            _, i, j = heapq.heappop(minHeap)
            if j + 1 < len(nums[i]):
                heapq.heappush(minHeap, (nums[i][j+1], i, j+1))
                mx = max(mx, nums[i][j+1])
                mn = minHeap[0][0]
                if mx - mn < maxRange - minRange:
                    maxRange, minRange = mx, mn

        return [minRange, maxRange]

