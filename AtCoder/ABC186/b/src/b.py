def main():
  h, w = map(int, input().split())
  a = [list(map(int, input().split())) for _ in range(h)]
  mi = 100
  for i in range(h):
    for j in range(w):
      mi = min(mi, a[i][j])
  ans = 0
  for i in range(h):
    for j in range(w):
      ans += a[i][j] - mi
  print(ans)

if __name__ == "__main__":
  main()
