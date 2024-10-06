class Solution:
    def areSentencesSimilar(self, sentence1: str, sentence2: str) -> bool:
        if len(sentence1) == len(sentence2):
            return sentence1 == sentence2
        
        s1 = sentence1.split(" ")
        s2 = sentence2.split(" ")
        prefixLen = 0
        suffixLen = 0

        m, n = len(s1), len(s2)

        if m > n:
            return self.areSentencesSimilar(sentence2, sentence1)
        
        i = 0

        while i < m and s1[i] == s2[i]:
            i += 1
        while i < m and s1[i] == s2[i + n - m]:
            i += 1
        
        return i == m

        # for w1, w2 in zip(s1, s2):
        #     if w1 != w2:
        #         break
        #     prefixLen += 1
        
        # for w1, w2 in zip(s1[::-1], s2[::-1]):
        #     if w1 != w2:
        #         break
        #     suffixLen += 1
        
        # common = prefixLen + suffixLen

        # return common == len(s1) or common == len(s2)

