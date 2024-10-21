class Solution:
    def parseBoolExpr(self, expression: str) -> bool:
        i = 0
        def parse() -> bool:
            nonlocal i
            # print(expression[i:])
            if expression[i] == 't':
                i += 1
                return True
            if expression[i] == 'f':
                i += 1
                return False
            if expression[i] == '!':
                i += 2
                ans = parse()
                i += 1
                return not ans
            isAnd = True if expression[i] == '&' else False
            ans = isAnd
            i += 2
            while expression[i] != ')':
                parsedExpr = parse()
                if isAnd:
                    ans = ans and parsedExpr
                else:
                    ans = ans or parsedExpr
                if expression[i] == ',':
                    i += 1
            i += 1
            return ans
        return parse()

