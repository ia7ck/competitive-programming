def main():
  n, w = map(int, input().split())
  ans = 0
  for i in range(0, 1001):
    ww = w * i
    if ww <= n:
      ans = max(ans, i)
  print(ans)

if __name__ == "__main__":
  main()
