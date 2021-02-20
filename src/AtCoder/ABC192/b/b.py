def small(c):
    return 'a' <= c <= 'z'


def big(c):
    return 'A' <= c <= 'Z'


def main():
    s = input()
    for i in range(len(s)):
        c = s[i]
        if i % 2 == 0:
            if big(c):
                print("No")
                return
        else:
            if small(c):
                print("No")
                return
    print("Yes")


if __name__ == '__main__':
    main()
