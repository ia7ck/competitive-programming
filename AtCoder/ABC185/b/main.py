def main():
  n, m, t = map(int, input().split())
  c = [0]
  for _ in range(m):
    a, b = map(int, input().split())
    c.append(a)
    c.append(b)
  c.append(t)
  d = []
  for i in range(1, len(c)):
    d.append(c[i] - c[i - 1])
  # print(c)
  # print(d)
  cur = n
  import sys
  for i in range(0, len(d)):
    if i % 2 == 0:
      cur -= d[i]
    else:
      cur += d[i]
      cur = min(cur, n)
    if cur <= 0:
      print("No")
      sys.exit(0)
  print("Yes")

if __name__ == "__main__":
  main()
