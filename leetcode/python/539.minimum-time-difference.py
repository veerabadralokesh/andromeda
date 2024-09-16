class Solution:
    def findMinDifference(self, timePoints: List[str]) -> int:
        minutes = [int(timePoint[:2]) * 60 + int(timePoint[3:]) for timePoint in timePoints]
        minutes.sort()
        ans = 1440

        for i in range(1, len(timePoints)):
            ans = min(ans, minutes[i]-minutes[i-1])
        
        ans = min(ans, 1440 - minutes[-1] + minutes[0])
        
        return ans

