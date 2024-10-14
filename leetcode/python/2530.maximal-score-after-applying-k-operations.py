class Solution:
    def maxKelements(self, nums: List[int], k: int) -> int:
        maxHeap = [-n for n in nums]
        heapq.heapify(maxHeap)

        ans = 0

        for _ in range(k):
            maxNum = heapq.heappop(maxHeap)
            ans -= maxNum
            heapq.heappush(maxHeap, maxNum//3)
        return ans

