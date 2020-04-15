def main():
  h, w, d = map(int, input().split())
  a = [list(map(int, input().split())) for i in range(h)]
  pts = [(0, 0) for i in range(h * w)]
  for i in range(h):
    for j in range(w):
      a[i][j] -= 1
      pts[a[i][j]] = (i, j)
  c = [[0 for j in range(h * w // d + 10)] for i in range(d)]
  for s in range(d):
    i = s
    j = s + d
    while j < h * w:
      iy, ix = pts[i]
      jy, jx = pts[j]
      c[s][(j - j % d) // d] = c[s][(i - i % d) // d] + abs(iy - jy) + abs(ix - jx)
      i += d
      j += d
  q = int(input())
  for i in range(q):
    l, r = map(int, input().split())
    l -= 1
    r -= 1
    s = l % d
    print(c[s][(r - r % d) // d] - c[s][(l - l % d) // d])

if __name__ == "__main__":
  main()
