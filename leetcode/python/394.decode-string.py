class Solution:
    def decodeString(self, s: str) -> str:
        ans = ""
        brackets = 0
        count = 0
        i = 0
        start = 0
        while i < len(s):
            c = s[i]
            if brackets == 0:
                if c.isdigit():
                    count = count * 10 + int(c)
                elif c == '[':
                    brackets += 1
                    start = i
                else:
                    ans += c
            else:
                if c == ']':
                    brackets -= 1
                if c == '[':
                    brackets += 1
                if brackets == 0:
                    repeating_string = self.decodeString(s[start+1 : i])
                    for _ in range(count):
                        ans += repeating_string
                    count = 0
            i += 1
        return ans

