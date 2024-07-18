class Solution:
    def twoSum(self, numbers: list[int], target: int) -> list[int]:
        i1 = 0
        i2 = len(numbers) - 1
        while i1 < i2:
            sum = numbers[i1] + numbers[i2]
            if sum == target:
                break
            elif sum < target:
                i1 += 1
            elif sum > target:
                i2 -= 1
        return [i1+1, i2+1]


if __name__ == "__main__":
    sol = Solution()
    numbers = [2,7,11,15]; target = 9
    print(sol.twoSum(numbers, target))
    numbers = [2,3,4]; target = 6
    print(sol.twoSum(numbers, target))
    numbers = [-1,0]; target = -1
    print(sol.twoSum(numbers, target))
