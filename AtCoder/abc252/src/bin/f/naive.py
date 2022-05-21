import collections

def accept(b, a):
    b_counter = collections.Counter(b)
    a_counter = collections.Counter(a)
    for x, c in a_counter.items():
        if b_counter[x] < c:
            return False
    return True

assert accept([1, 2, 2, 3], [])
assert accept([1, 2, 2, 3], [1])
assert accept([1, 2, 2, 3], [1, 2])
assert accept([1, 2, 2, 3], [1, 2, 2])
assert not accept([1, 2, 2, 3], [1, 2, 2, 2])
assert not accept([1, 2, 2, 3], [4])

answer = 1_000_000_000

def dfs(b, cost, a, memory):
    global answer
    if cost >= answer:
        return
    if accept(b, a):
        if answer >= cost:
            # print(memory)
            answer= cost
        return
    for i, k in enumerate(b):
        for x in range(1, k):
           new_b = [b[j] for j in range(0, len(b)) if j != i] + [x, k - x]
           memory.append((sorted(new_b), cost + k))
           dfs(sorted(new_b), cost + k, a, memory)
           memory.pop()
def main():
    n, l = map(int, input().split())
    a = list(map(int, input().split()))

    assert l <= 10
    
    dfs([l], 0, a, [([l], 0)])
    print(answer)
    


if __name__ == '__main__':
    main()
