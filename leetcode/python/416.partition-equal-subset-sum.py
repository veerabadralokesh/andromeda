

def canPartition(self, nums: List[int]) -> bool:
    
        tsum = sum(nums)
        if tsum % 2 == 1: return False
        memo = {}

        def dfs(subsetsum, csum, index):
            if csum == subsetsum:
                return True

            if csum > subsetsum or index == len(nums):
                return False
            
            if csum + nums[index] in memo:
                return memo[csum + nums[index]]

            result =  dfs(
                subsetsum, csum + nums[index], index+1
                ) or dfs(
                    subsetsum, csum, index+1
                    )
            memo[csum + nums[index]] = result

            return result

        subsetsum = tsum // 2
        return dfs(subsetsum, 0, 0)
