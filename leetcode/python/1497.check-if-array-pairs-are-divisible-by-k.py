class Solution:
    def canArrange(self, arr: List[int], k: int) -> bool:
        rems = [0] * k

        for n in arr:
            rems[((n % k) + k) % k] += 1
        
        if rems[0] & 1 != 0:
            return False
        
        for i in range(1, 1 + k // 2):
            if rems[i] != rems[k-i]:
                return False
        
        return True

