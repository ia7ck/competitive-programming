if __name__ == '__main__':
    n, x = map(int, input().split())
    a = list(map(int, input().split()))
    for y in a:
        if x != y:
            print(y)
    # " ".join(a)
