class Solution:
    def closeStrings(self, word1: str, word2: str) -> bool:
        if len(word1) != len(word2):
            return False
        charCount = ord('z') + 1
        chars1 = [0] * charCount
        chars2 = [0] * charCount
        
        for c in word1:
            chars1[ord(c)] += 1
        
        for c in word2:
            if chars1[ord(c)] == 0:
                return False
            chars2[ord(c)] += 1
        
        chars1.sort()
        chars2.sort()

        return chars1 == chars2

""" """

class Solution:
    def closeStrings(self, word1: str, word2: str) -> bool:
        list1 =[] 
        list2 = [] 
        for i in set(word1):
            list1.append(word1.count(i)) 
            list2.append(word2.count(i))
        list1.sort()
        list2.sort()
        return list1 == list2

