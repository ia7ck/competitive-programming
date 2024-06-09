def main():
    n = int(input())
    a = list(map(int, input().split()))
    
    a = [i - 1 for i in a]

    ans = 0
    for s in range(n):
        x = s
        hist = []
        while x not in hist:
            hist.append(x)
            x = a[x]
        ans += len(hist)
    
    print(ans)

if __name__ == '__main__':
    main()
