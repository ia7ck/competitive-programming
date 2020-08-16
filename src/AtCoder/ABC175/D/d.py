def main():
    import numpy as np
    N, K = map(int, input().split())
    P = [int(x) - 1 for x in input().split()]
    P = np.array(P, np.int64)
    C = np.array(input().split(), np.int64)
    ans = -(10 ** 18)
    for i in range(N):
        j = i
        S = 0
        L = 0
        for _ in range(K):
            j = P[j]
            S += C[j]
            L += 1
            ans = max(ans, S)
            if j == i:
                break
        if j == i:
            M = max(0, K // L - 1)
            T = S * M
            for _ in range(M * L, K):
                j = P[j]
                T += C[j]
                ans = max(ans, T)
    print(ans)


if __name__ == "__main__":
  main()
