import heapq

class Solution:
    def findCheapestPrice(self, n: int, flights: List[List[int]], src: int, dst: int, k: int) -> int:
        grid=[[] for i in range(n)]
        for frm,to,cst in flights:
            grid[frm].append((cst,to))
        
        st=[(0,0,src)]
        heapq.heapify(st)
        css=[(float("infinity"),float("infinity"))]*n
        css[src]=(0,0)
        while st:
            cst,stop,x=heapq.heappop(st)
            if x==dst:
                return cst
            if stop<=k:
                for ct,to in grid[x]:
                    if css[to][0]>ct+cst or css[to][1]>stop+1:
                        css[to]=(ct+cst,stop+1)
                        heapq.heappush(st,(cst+ct,stop+1,to))

        return -1

class Solution:
  def findCheapestPrice(self, n: int, flights: List[List[int]], src: int, dst: int, k: int) -> int:
    graph = [[] for _ in range(n)]

    for u, v, w in flights:
      graph[u].append((v, w))

    return self._dijkstra(graph, src, dst, k)

  def _dijkstra(self, graph: List[List[Tuple[int, int]]], src: int, dst: int, k: int) -> int:
    dist = [[math.inf for _ in range(k + 2)] for _ in range(len(graph))]

    dist[src][k + 1] = 0
    minHeap = [(dist[src][k + 1], src, k + 1)]  # (d, u, stops)

    while minHeap:
      d, u, stops = heapq.heappop(minHeap)
      if u == dst:
        return d
      if stops == 0:
        continue
      for v, w in graph[u]:
        if d + w < dist[v][stops - 1]:
          dist[v][stops - 1] = d + w
          heapq.heappush(minHeap, (dist[v][stops - 1], v, stops - 1))

    return -1
