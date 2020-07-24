def solve(n, arr):
  a = [x for x in arr]
  res = 0
  s = [0 for i in range(n)]
  if a[0] <= 0:
    res += a[0] * (-1) + 1
    a[0] += a[0] * (-1) + 1
  s[0] = a[0]
  for i in range(1, n):
    s[i] = s[i - 1] + a[i]
    if s[i - 1] > 0 and s[i] >= 0:
      res += s[i - 1] + a[i] + 1
      a[i] -= s[i - 1] + a[i] + 1
    elif s[i - 1] < 0 and s[i] <= 0:
      res += 1 - s[i - 1] - a[i]
      a[i] += 1 - s[i - 1] - a[i]
    s[i] = s[i - 1] + a[i]
  return res

def main():
  n = int(input())
  a = list(map(int, input().split()))

  print(min(solve(n, a), solve(n, list(map(lambda x: x * (-1), a)))))

if __name__ == "__main__":
  main()
