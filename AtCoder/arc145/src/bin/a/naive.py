import random

def palindrome(s):
    return s == list(reversed(s))

def main():
    n = int(input())
    s = list(input())
    tset = set()
    for i in range(100):
        t = s.copy()
        for j in range(100):
            tset.add("".join(t))
            if palindrome(t):
                print("Yes")
                return
            k = random.randint(0, n - 2)
            t[k] = 'A'
            t[k + 1] = 'B'
    print("No")

if __name__ == '__main__':
    main()
