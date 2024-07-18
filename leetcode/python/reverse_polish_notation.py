class Solution:
    def evalRPN(self, tokens: list[str]) -> int:
        stack = []
        for token in tokens:
            if token in ["+", "-", "*", "/"]:
                num2 = stack.pop()
                num1 = stack.pop()
                if token == "+":
                    stack.append(num1+num2)
                if token == "-":
                    stack.append(num1-num2)
                if token == "*":
                    stack.append(num1*num2)
                if token == "/":
                    stack.append(int(num1/num2))
            else:
                stack.append(int(token))
        return stack[-1]



if __name__=="__main__":
    sol = Solution()
    tokens = ["2","1","+","3","*"]
    print(sol.evalRPN(tokens))
    tokens = ["4","13","5","/","+"]
    print(sol.evalRPN(tokens))
    tokens = ["10","6","9","3","+","-11","*","/","*","17","+","5","+"]
    print(sol.evalRPN(tokens))
    tokens = ["4","-2","/","2","-3","-","-"]
    print(sol.evalRPN(tokens))
