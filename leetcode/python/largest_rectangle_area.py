class Solution:
    
    def area(self, heights, start, end):
        if start >= end:
            return heights[start]
        if (start, end) in self.dict:
            return self.dict[(start, end)]
        la = self.area(heights, start, end-1)
        ra = self.area(heights, start+1, end)
        area = max([0])
        self.dict[(start, end)] = area
        return area

    def largestRectangleArea(self, heights: list[int]) -> int:
        max_area = 0
        start = 0
        end = len(heights) - 1
        self.dict = {}
        for i, h in enumerate(heights):
            if h == 0:
                end = i-1
                max_area = max(max_area, self.area(heights, start, end))
                start = i + 1

        return max_area


if __name__ == "__main__":
    sol = Solution()
    # heights = [2,1,5,6,2,3]
    # print(sol.largestRectangleArea(heights))
    # heights = [2,4]
    # print(sol.largestRectangleArea(heights))
    # heights = [0,9]
    # print(sol.largestRectangleArea(heights))
    heights = [0,1,0,2,1,0,1,3,2,1,2,1]
    print(sol.largestRectangleArea(heights))
