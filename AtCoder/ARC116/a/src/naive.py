def factors(n: int) -> [int]:
    return [x for x in range(1, n + 1) if n % x == 0]


def solve(n: int):
    fs = factors(n)
    odd = len([x for x in fs if x % 2 == 1])
    even = len([x for x in fs if x % 2 == 0])
    if odd == even:
        print("Same")
    elif odd > even:
        print("Odd")
    else:
        print("Even")


def main():
    t = int(input())
    for _ in range(t):
        n = int(input())
        solve(n)


if __name__ == '__main__':
    main()
