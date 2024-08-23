class Solution:
    def asteroidCollision(self, asteroids: List[int]) -> List[int]:
        stack = []
        for a in asteroids:
            exploded = False
            while stack and stack[-1] > 0 and a < 0 and not exploded:
                if stack[-1] > abs(a):
                    exploded = True
                elif stack[-1] == abs(a):
                    exploded = True
                    stack.pop()
                else:
                    stack.pop()
            if not exploded:
                stack.append(a)
        return stack

