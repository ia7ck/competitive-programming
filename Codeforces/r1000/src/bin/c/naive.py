class UnionFind:
    def __init__(self, n):
        self.parent = [i for i in range(n)]
        self.rank = [0 for _ in range(n)]

    def find(self, u):
        if self.parent[u] == u:
            return u
        self.parent[u] = self.find(self.parent[u])
        return self.parent[u]

    def union(self, u, v):
        pu, pv = self.find(u), self.find(v)
        if pu == pv:
            return False
        if self.rank[pu] < self.rank[pv]:
            pu, pv = pv, pu
        self.parent[pv] = pu
        if self.rank[pu] == self.rank[pv]:
            self.rank[pu] += 1
        return True


def solve(n, edges):
    ans = 0
    for u in range(n):
        for v in range(u + 1, n):
            uf = UnionFind(n)
            for x, y in edges:
                if x != u and x != v and y != u and y != v:
                    uf.union(x, y)
            # count connected components
            cc = set()
            for i in range(n):
                if i != u and i != v:
                    cc.add(uf.find(i))
            ans = max(ans, len(cc))
    print(ans)

t = int(input())
for i in range(t):
    n = int(input())
    edges = []
    for j in range(n - 1):
        u, v = map(int, input().split())
        edges.append((u - 1, v - 1))
    solve(n, edges)
