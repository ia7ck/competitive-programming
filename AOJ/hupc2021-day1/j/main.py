import collections


def main():
    n = int(input())
    a = list(map(int, input().split()))

    mod = 998244353
    fact = [0] * (n + 2)
    fact[0] = 1
    for i in range(1, n + 2):
        fact[i] = fact[i - 1] * i % mod
    inv = [0] * (n + 2)
    inv[1] = 1
    for i in range(2, n + 2):
        # mod = (mod//i)*i + (mod%i)
        inv[i] = (mod - ((mod // i) * inv[mod % i]) % mod) % mod
    ifact = [0] * (n + 2)
    ifact[0] = 1
    for i in range(1, n + 2):
        ifact[i] = ifact[i - 1] * inv[i] % mod

    def binom(x, y):
        if x < y:
            return 0
        return fact[x] * ifact[y] % mod * ifact[x - y] % mod

    b = [6] + [x for x in a if x not in [1, 5, 7]] + [6]

    ans = 1
    six_index = [i for i, x in enumerate(b) if x == 6]
    for i in range(1, len(six_index)):
        l = six_index[i - 1] + 1
        r = six_index[i]
        odd = len([x for x in b[l:r] if x % 2 == 1])
        even = len([x for x in b[l:r] if x % 2 == 0])
        ans *= binom(odd + even, odd)
        ans %= mod

    counter = collections.Counter(a)
    ans *= binom(n - counter[1], counter[5] + counter[7]) * binom(counter[5] + counter[7], counter[5]) % mod
    ans %= mod
    ans *= binom(n, counter[1])
    ans %= mod
    print(ans)


if __name__ == '__main__':
    main()
