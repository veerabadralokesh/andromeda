class Solution:
    def maxProbability(self, n: int, edges: List[List[int]], succProb: List[float], start_node: int, end_node: int) -> float:
        graph = [[] for _ in range(n)]
        visited = [False] * n
        for [u, v], prob in zip(edges, succProb):
            graph[u].append([v, prob])
            graph[v].append([u, prob])

        heap = []
        heapq.heapify(heap)
        heapq.heappush(heap, (-1, start_node))

        while heap:
            prob, u = heapq.heappop(heap)
            visited[u] = True
            if u == end_node:
                return -prob
            for v, proba in graph[u]:
                if visited[v]:
                    continue
                heapq.heappush(heap, (prob * proba, v))
        return 0.0

