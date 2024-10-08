class Solution:
    def minSwaps(self, s: str) -> int:
        stack = []
        swaps = 0
        swapPosition = None

        for i, c in enumerate(s):
            if c == '[':
                stack.append(c)
            elif stack:
                stack.pop()
            else:
                if swapPosition is None:
                    swaps += 1
                    swapPosition = i
                else:
                    swapPosition = None
        
        return swaps

""" """

class Solution:
    def minSwaps(self, s: str) -> int:
        stack = 0
        swaps = 0
        swapPosition = None

        for i, c in enumerate(s):
            if c == '[':
                stack += 1
            elif stack:
                stack -= 1
            else:
                if swapPosition is None:
                    swaps += 1
                    swapPosition = i
                else:
                    swapPosition = None
        
        return swaps

