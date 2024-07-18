# import re
# class Solution:
#     def isPalindrome(self, s: str) -> bool:
#         s = ''.join(re.findall("[a-z0-9]*", s.lower()))
#         l = len(s)
#         for i in range(l//2):
#             if s[i] != s[l-i-1]:
#                 return False
#         return True

class Solution:
    def isPalindrome(self, s: str) -> bool:
        s = s.lower()
        start = 0
        end = len(s) - 1
        while start < end:
            while (start < end and not s[start].isalnum()):
                start += 1
            while (start < end and not s[end].isalnum()):
                end -= 1
            if s[start] != s[end]:
                return False
            else:
                start += 1
                end -= 1
        return True

if __name__=="__main__":
    sol = Solution()
    s = "A man, a plan, a canal: Panama"
    print(sol.isPalindrome(s))
    s = "race a car"
    print(sol.isPalindrome(s))
    s = " "
    print(sol.isPalindrome(s))