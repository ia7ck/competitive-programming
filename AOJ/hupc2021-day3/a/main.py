def main():
    (h1, w1) = map(int, input().split())
    (h2, w2) = map(int, input().split())
    ans = ((h1 + h2 - 1) // h2) * ((w1 + w2 - 1) // w2)
    print(ans)


if __name__ == '__main__':
    main()
