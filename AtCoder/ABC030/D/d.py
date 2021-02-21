def main():
    n, start = map(int, input().split())
    k = int(input())
    b = list(map(int, input().split()))

    b = [x - 1 for x in b]
    last = [0] * n
    cur = start - 1
    for i in range(k):
        if last[cur] >= 1:
            period = (i + 1) - last[cur]
            remain_step = k - i
            break
        last[cur] = i + 1
        cur = b[cur]
        # print(last)
    else:
        print(cur + 1)
        return
    # print(period)
    # print(remain_step)
    for _ in range(remain_step % period):
        cur = b[cur]
    print(cur + 1)


if __name__ == "__main__":
    main()
