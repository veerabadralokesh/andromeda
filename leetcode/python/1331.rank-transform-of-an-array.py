class Solution:
    def arrayRankTransform(self, arr: List[int]) -> List[int]:
        if not arr:
            return arr
        
        orig = [n for n in arr]

        arr.sort()

        ranks = {arr[0]: 1}
        rank = 2

        for i in range(1,len(arr)):
            if arr[i] != arr[i-1]:
                ranks[arr[i]] = rank
                rank += 1
        
        return [ranks[n] for n in orig]

