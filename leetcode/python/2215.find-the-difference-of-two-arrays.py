class Solution:
    def findDifference(self, nums1: List[int], nums2: List[int]) -> List[List[int]]:
        nums1f = [False] * 2001
        for n in nums1:
            nums1f[n] = True
        nums2f = [False] * 2001
        ans = [[], []]
        for n in nums2:
            nums2f[n] = True
            if not nums1f[n]:
                nums1f[n] = True
                ans[1].append(n)
        for n in nums1:
            if not nums2f[n]:
                nums2f[n] = True
                ans[0].append(n)
        return ans

