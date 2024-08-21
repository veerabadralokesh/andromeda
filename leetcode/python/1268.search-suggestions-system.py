class Solution:
    def suggestedProducts(self, products: List[str], searchWord: str) -> List[List[str]]:
        products.sort()
        words = [[] for _ in searchWord]
        iwords = products
        for i in range(len(searchWord)):
            for p in iwords:
                if len(p) > i and p[i] == searchWord[i]:
                    words[i].append(p)
            iwords = words[i]
            words[i] = words[i][:3]
        return words
 
