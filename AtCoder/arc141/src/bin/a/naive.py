def f(n):
    s = str(n)
    for i in range(len(s)):
        t = s[:(i + 1)]
        l = len(t)
        if len(s) % l == 0 and len(s) // l >= 2:
            x = ""
            for _ in range(len(s) // l):
                for ch in t:
                    x += ch
            assert len(x) == len(s)
            if x == s:
                return True
    return False

def solve(n):
    assert 11 <= n <= 10000
    for k in reversed(range(1, n + 1)):
        if f(k):
            print(k)
            return
    assert False

def main():
    t = int(input())
    for _ in range(t):
        n = int(input())
        solve(n)

if __name__ == '__main__':
    main()
