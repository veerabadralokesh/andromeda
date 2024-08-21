class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        products = [1 for _ in nums]
        product = 1
        for i, n in enumerate(nums):
            products[i] = product
            product = product * n
        product = 1
        N = len(nums)
        for i, n in enumerate(nums[::-1]):
            products[N-i-1] = products[N-i-1] * product
            product = product * n
        return products

