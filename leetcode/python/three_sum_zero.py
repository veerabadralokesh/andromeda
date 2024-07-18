class Solution:
    def threeSum(self, nums: list[int]) -> list[list[int]]:
        sol = []
        _min = min(nums)
        _nums = [i-_min for i in nums]
        zero_indices = []
        pairs = {}
        return sol



if __name__ == "__main__":
    sol = Solution()
    nums = [-1,0,1,2,-1,-4]
    print(sol.threeSum(nums))
    nums = [0,1,1]
    print(sol.threeSum(nums))
    nums = [0,0,0]
    print(sol.threeSum(nums))
