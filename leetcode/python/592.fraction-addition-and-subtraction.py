class Solution:
    def fractionAddition(self, expression: str) -> str:
        numerator = []
        denominator = []
        is_num = True
        sign = 1
        num = 0
        denom = 0
        for c in expression:
            if c == '+' or c == '-':
                sign = -1 if c == '-' else 1
                if denom > 0:
                    denominator.append(denom)
                    is_num = True
                    denom = 0
            elif c == '/':
                is_num = False
                numerator.append(sign * num)
                sign = 1
                num = 0
            else:
                if is_num:
                    num = num * 10 + int(c)
                else:
                    denom = denom * 10 + int(c)
        denominator.append(denom)
        lcm = denominator[0]
        for i in range(1, len(denominator)):
            lcm = self.lcm(lcm, denominator[i])
        
        num = 0
        for i, n in enumerate(numerator):
            num += (n * lcm // denominator[i])
        
        x = self.gcd(lcm, abs(num))
        while abs(num) > 0 and x > 1:
            num = num // x
            lcm = lcm // x
            x = self.gcd(lcm, abs(num))

        if num == 0:
            lcm = 1
        
        return f"{num}/{lcm}"
    
    def gcd(self, a: int, b: int) -> int:
        if b == 0:
            return a
        return self.gcd(b, a % b)
    
    def lcm(self, a: int, b: int) -> int:
        return a * b // gcd(a, b)


