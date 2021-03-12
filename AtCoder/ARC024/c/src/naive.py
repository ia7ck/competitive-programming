def main():
    n, k = map(int, input().split())
    s = input()

    for i in range(n):
        for j in range(i + k, n):
            if j + k <= n:
                t = s[i:i + k]
                u = s[j:j + k]
                if sorted(t) == sorted(u):
                    print("YES")
                    return
    print("NO")


if __name__ == '__main__':
    main()
