def main():
    n, s = map(int, input().split())
    a = list(map(int, input().split()))

    for l in range(n):
        i = l
        t = 0
        while t < s:
            t += a[i % n]
            i += 1
        if t == s:
            print('Yes')
            return
    
    print('No')

if __name__ == '__main__':
    main()
