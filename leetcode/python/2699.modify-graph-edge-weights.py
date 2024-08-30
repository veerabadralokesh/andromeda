class Solution:
    def modifiedGraphEdges(self, n: int, edges: List[List[int]], source: int, destination: int, target: int) -> List[List[int]]:

        maxEdgeWeight = 2_000_000_000
        
        graph = [[] for _ in range(n)]

        negEdges = []
        for i, (u, v, w) in enumerate(edges):
            if w == -1:
                negEdges.append(i)
                continue
            graph[u].append((v, w))
            graph[v].append((u, w))
        
        distToDest = self.dijkstra(graph, source, destination)
        if distToDest < target:
            # print("returning")
            return []
        if distToDest == target:
            for i in negEdges:
                edges[i][2] = maxEdgeWeight
            return edges
        
        for idx, i in enumerate(negEdges):
            edges[i][2] = 1
            u, v, w = edges[i]
            graph[u].append((v, 1))
            graph[v].append((u, 1))
            distToDest = self.dijkstra(graph, source, destination)
            # print(distToDest)
            if distToDest <= target:
                edges[i][2] += target - distToDest
                for j in negEdges[idx+1:]:
                    edges[j][2] = maxEdgeWeight
                return edges
        return []

    def dijkstra(self, graph: List[List[int]], src: int, dest: int) -> int:
        dist = [float('inf')] * len(graph)
        minHeap = []

        dist[src] = 0
        heapq.heappush(minHeap, (0, src))

        while minHeap:
            d, u = heapq.heappop(minHeap)
            if d > dist[u]:
                continue
            for v, w in graph[u]:
                if d + w < dist[v]:
                    dist[v] = d + w
                    heapq.heappush(minHeap, (dist[v], v))
        
        return dist[dest]

