class Solution:
    def minSubarray(self, nums: List[int], p: int) -> int:
        rem = sum(nums) % p
        if rem == 0:
            return 0
        prefix = 0
        idxs = {0: -1}
        ans = len(nums)
        for i, n in enumerate(nums):
            prefix = (prefix + n) % p
            target = (prefix - rem + p) % p
            if target in idxs:
                ans = min(ans, i - idxs[target])
            idxs[prefix] = i
        return -1 if ans == len(nums) else ans

