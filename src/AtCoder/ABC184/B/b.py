def main():
  n, x = map(int, input().split())
  s = input()
  for c in s:
    if c == 'o':
      x +=1
    else:
      if x > 0:
        x -= 1
  print(x)

if __name__ == "__main__":
  main()
