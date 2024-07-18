class MinStack:

    def __init__(self):
        self.stack = []
        self.min_vals = []

    def push(self, val: int) -> None:
        self.stack.append(val)
        if len(self.min_vals) == 0:
            self.min_vals.append(val)
        else:
            self.min_vals.append(min(val, self.min_vals[-1]))

    def pop(self) -> None:
        self.stack.pop(-1)
        self.min_vals.pop(-1)

    def top(self) -> int:
        return self.stack[-1]

    def getMin(self) -> int:
        return self.min_vals[-1]


# Your MinStack object will be instantiated and called as such:
# obj = MinStack()
# obj.push(val)
# obj.pop()
# param_3 = obj.top()
# param_4 = obj.getMin()