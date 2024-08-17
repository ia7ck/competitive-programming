def dfs(i, freq, lr):
    if i == len(lr):
        for f in freq:
            if f != 1:
                return False
        return True
    l, r = lr[i]
    result = False
    result |= dfs(i + 1, freq, lr)
    for j in range(l - 1, r):
        freq[j] += 1
    result |= dfs(i + 1, freq, lr)
    for j in range(l - 1, r):
        freq[j] -= 1
        freq[j] -= 1
    result |= dfs(i + 1, freq, lr)
    for j in range(l - 1, r):
        freq[j] += 1
    return result

def main():
    n, q = map(int, input().split())
    lr = [tuple(map(int, input().split())) for _ in range(q)]
    
    freq = [0] * n
    ans = dfs(0, freq, lr)
    if ans:
        print("Yes")
    else:
        print("No")

if __name__ == '__main__':
    main()
