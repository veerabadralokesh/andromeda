class Solution:
    def missingRolls(self, rolls: List[int], mean: int, n: int) -> List[int]:
        rem = (len(rolls) + n) * mean - sum(rolls)
        if rem > n * 6 or rem < n:
            return []
        ans = [rem//n] * n
        rem = rem % n
        for i in range(rem):
            ans[i] += 1
        return ans

