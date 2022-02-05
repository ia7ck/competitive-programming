def solve(a, s):
    for x in range(s + 1):
        y = s - x
        if (x & y) == a:
            print("Yes")
            return
    print("No")

def main():
    t = int(input())
    for _ in range(t):
        a, s = map(int, input().split())
        solve(a, s)

if __name__ == '__main__':
    main()