k = int(input())
s = input()
t = input()

dp = [[float('inf')] * (len(t) + 1) for i in range(len(s) + 1)]
for i in range(len(s) + 1):
    dp[i][0] = i
for j in range(len(t) + 1):
    dp[0][j] = j

for i in range(len(s)):
    for j in range(len(t)):
        dp[i + 1][j + 1] = min(dp[i + 1][j] + 1, dp[i][j + 1] + 1)
        if s[i] == t[j]:
            dp[i + 1][j + 1] = min(dp[i + 1][j + 1], dp[i][j])
        else:
            dp[i + 1][j + 1] = min(dp[i + 1][j + 1], dp[i][j] + 1)

if dp[len(s)][len(t)] <= k:
    print("Yes")
else:
    print("No")
