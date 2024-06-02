import itertools

def main():
    n, k = map(int, input().split())
    a = list(map(int, input().split()))
    for p in itertools.permutations(a):
        cum_sum = [0] * (n + 1)
        for i in range(n):
            cum_sum[i + 1] = cum_sum[i] + p[i]
        for i in range(n + 2):
            if all([x < k for x in cum_sum[:i]]) and all([x >= k for x in cum_sum[i:]]):
                print("Yes")
                # print(*p)
                return
    print("No")

if __name__ == '__main__':
    main()
