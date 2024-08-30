class Solution:
    @cache
    def diffWaysToCompute(self, expression: str) -> List[int]:
        ans = []

        for i, c in enumerate(expression):
            if c == '+' or c == '-' or c == '*':
                for prefix in self.diffWaysToCompute(expression[:i]):
                    for suffix in self.diffWaysToCompute(expression[i+1:]):
                        if c == '+':
                            ans.append(prefix + suffix)
                        elif c == '-':
                            ans.append(prefix - suffix)
                        else:
                            ans.append(prefix * suffix)
        
        return ans or [int(expression)]

