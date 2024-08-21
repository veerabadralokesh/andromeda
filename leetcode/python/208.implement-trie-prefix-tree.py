class Trie:

    def __init__(self):
        self.trie = {}

    def insert(self, word: str) -> None:
        node = self.trie
        for c in word:
            if c not in node:
                node[c] = {}
            node = node[c]
        node['end'] = 1
        
    def search(self, word: str) -> bool:
        node = self.trie
        for c in word:
            node = node.get(c)
            if node is None:
                return False
        return node.get('end', 0) > 0
        

    def startsWith(self, prefix: str) -> bool:
        node = self.trie
        for c in prefix:
            node = node.get(c)
            if node is None:
                return False
        return True
        


# Your Trie object will be instantiated and called as such:
# obj = Trie()
# obj.insert(word)
# param_2 = obj.search(word)
# param_3 = obj.startsWith(prefix)

