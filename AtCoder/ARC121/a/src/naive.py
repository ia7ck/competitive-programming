def main():
    n = int(input())
    xy = []
    for _ in range(n):
        x, y = list(map(int, input().split()))
        xy.append((x, y))
    dist = []
    for i in range(n):
        x, y = xy[i]
        for j in range(i):
            xx, yy = xy[j]
            d = max(abs(x - xx), abs(y - yy))
            dist.append(d)
    dist.sort()
    dist.reverse()
    # print(dist)
    print(dist[1])


if __name__ == '__main__':
    main()
