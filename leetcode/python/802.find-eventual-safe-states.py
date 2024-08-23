class Solution:
    def eventualSafeNodes(self, graph: List[List[int]]) -> List[int]:
        n = len(graph)
        counts = [len(graph[i]) for i in range(n)]
        incoming = [[] for _ in range(n)]
        q = deque([])
        for i, out in enumerate(graph):
            for j in out:
                incoming[j].append(i)
            if len(out) == 0:
                q.append(i)
        
        ans = []

        while q:
            node = q.popleft()
            ans.append(node)
            for i in incoming[node]:
                counts[i] -= 1
                if counts[i] == 0:
                    q.append(i)

        ans.sort()
        return ans

