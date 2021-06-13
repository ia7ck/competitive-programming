def main():
    n, m, k = map(int, input().split())

    ans = 0
    for bits in range(1 << (n + m)):
        s = ""
        for i in range(n + m):
            if bits >> i & 1:
                s += 'w'
            else:
                s += 'b'
        if s.count('w') == n and s.count('b') == m:
            ok = True
            w, b = 0, 0
            for i in range(n + m):
                if s[i] == 'w':
                    w += 1
                else:
                    b += 1
                if w > b + k:
                    ok = False
                    break
            if ok:
                ans += 1
    print(ans)


if __name__ == '__main__':
    main()
