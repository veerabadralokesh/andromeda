class Solution:
    def ladderLength(self, beginWord: str, endWord: str, wordList: List[str]) -> int:
        wordSet = set(wordList)

        if endWord not in wordSet:
            return 0
        
        q = deque([beginWord])

        transformations = 0
        while q:
            transformations += 1
            for _ in range(len(q)):
                origWord = q.popleft()
                charList = list(origWord)
                for i, orig in enumerate(charList):
                    for c in "abcdefghijklmnopqrstuvwxyz":
                        charList[i] = c
                        word = "".join(charList)
                        if word == endWord:
                            return transformations + 1
                        if origWord == word or word not in wordSet:
                            continue
                        q.append(word)
                        wordSet.remove(word)
                    charList[i] = orig
        return 0

