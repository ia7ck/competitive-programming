import sys
sys.setrecursionlimit(100000)

def dfs(i, g, visited):
  visited.add(i)
  for j in g[i]:
    if not (j in visited):
      dfs(j, g, visited)

def main():
  n, m = map(int, input().split())
  p = (list(map(int, input().split())))
  g = [[] for i in range(n + 1)]
  for i in range(m):
    x, y = map(int, input().split())
    g[x].append(y)
    g[y].append(x)
  seen = [False for i in range(n + 1)]
  ans = 0
  for i in range(1, n):
    if not seen[i]:
      visited = set()
      dfs(i, g, visited)
      for j in visited:
        seen[j] = True
        if p[j - 1] in visited:
          ans += 1
  print(ans)

if __name__ == "__main__":
  main()
