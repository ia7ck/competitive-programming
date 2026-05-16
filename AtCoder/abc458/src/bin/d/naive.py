x = int(input())
q = int(input())

x = [x]
for _ in range(q):
    a, b = map(int, input().split())
    x.append(a)
    x.append(b)

    print(sorted(x)[len(x) // 2])
