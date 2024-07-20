def main():
    n = int(input())
    for i in range(0, 1000000000):
        if str(i) == str(i)[::-1]:
            n -= 1
            if n == 0:
                print(i)
                return

if __name__ == '__main__':
    main()
