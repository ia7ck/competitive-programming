def shift(s, n):
    n %= 26
    t = ""
    for i in range(len(s)):
        if ord(s[i]) + n <= ord('z'):
            t += chr(ord(s[i]) + n)
        else:
            t += chr(ord('a') + (ord(s[i]) + n - ord('z') - 1))
    return t


def main():
    s = input()
    stack = []
    stack_sum = 0
    i = 0
    ans = []
    while i < len(s):
        if s[i].isdigit():
            x = int(s[i])
            stack.append(x)
            stack_sum += x
            assert s[i + 1] == '('
            if s[i + 2].isdigit():
                i += 2
            else:
                assert s[i + 2].isalpha()
                j = i + 2
                while s[j].isalpha():
                    j += 1
                t = shift(s[i + 2:j], stack_sum)
                ans.append(t)
                i = j
        elif s[i].isalpha():
            j = i
            while j < len(s) and s[j].isalpha():
                j += 1
            t = shift(s[i:j], stack_sum)
            ans.append(t)
            i = j
        else:
            assert s[i] == ')'
            top = stack.pop()
            stack_sum -= top
            i += 1
    print("".join(ans))


if __name__ == '__main__':
    main()
