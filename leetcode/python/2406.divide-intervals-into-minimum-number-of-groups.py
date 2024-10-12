class Solution:
    def minGroups(self, intervals: List[List[int]]) -> int:
        intervals.sort()
        heap = [] # minheap
        for i in intervals:
            if heap and heap[0] < i[0]:
                heapq.heappop(heap)
            heapq.heappush(heap, i[1])
        
        return len(heap)

