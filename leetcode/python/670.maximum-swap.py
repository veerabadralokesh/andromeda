class Solution:
    def maximumSwap(self, num: int) -> int:
        # nums = list(map(int, list(str(num))))
        nums = list(str(num))
        changed = False
        ans = num
        for i in range(len(nums)):
            for j in range(i+1, len(nums)):
                if nums[j] > nums[i]:
                    numsc = [i for i in nums]
                    numsc[i], numsc[j] = numsc[j], numsc[i]
                    x = int("".join(numsc))
                    if x > ans:
                        ans = x
        return ans

""" """

class Solution:
    def maximumSwap(self, num: int) -> int:
        s = list(str(num))
        position = {c: i for i, c in enumerate(s)}

        for i, c in enumerate(s):
            for digit in reversed(string.digits):
                if digit <= c:
                    break
                if digit in position and position[digit] > i:
                    s[i], s[position[digit]] = digit, s[i]
                    return int("".join(s))

        return num

