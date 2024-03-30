def main():
    n, a, b = map(int, input().split())
    d = list(map(int, input().split()))

    for s in range(1, a + b + 1):
        ok = True
        for x in d:
            t = s + x
            while t > a + b:
                t -= a + b
            if t > a:
                ok = False
                break
        if ok:
            print("Yes")
            return
    print("No")

if __name__ == '__main__':
    main()
