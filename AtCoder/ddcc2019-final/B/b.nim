import strutils, sequtils
proc main() =
  let
    nkr = stdin.readLine.split.map(parseBiggestInt)
    (n, k, r) = (nkr[0].int, nkr[1].int, nkr[2])
  var
    rem = r
    ans = newSeq[int](0)
    seen = newSeq[bool](n)
  for ri in 0..<n:
    let i = n-ri              # n, n-1, ..., の順でみる
    if 0<i-k and i-k<=rem:
      rem-=(i-k)
      ans.add(i)
      seen[i-1] = true
    else:
      discard
  if rem>0:
    echo "No Luck"
  else:
    for i in 0..<n:
      if not seen[i]:
        ans.add(i+1)
    echo ans.mapIt(string, $(it)).join(" ")
main() #[]#

# 4 3 2 1
# K=1
# => R=6
# K=2
# => R=3
# K=3
# => 1

# A_1=xとする
# max(0, x-K)個は条件を満たす組の個数に寄与する
# - これがRを超えたらその時点でダメ
# - ちょうどRにできたら勝ち？
#   - 残りを昇順にならべれば個数は増えないので
# - R超えないギリギリのやつを選んでいけばいい？（さすがに嘘では）
# A_1=x, A_2=yとする
# - x>y：max(0, y-K)個が...
# - x<y：
#   - y-K<x：max(0, y-K)
#   - y-K>=x：上の -1 （xは左にあるので）

# K=1ならinversionの個数?

# Sample 1
# N=5, K=2, R=4
# 5 => +3
# 3 => +1
# 1 2 4
# ans = 5 3 1 2 4 ?

# Sample 3
# N=10, K=3, R=22
# ans = [10, 9, 8, 7, 1, 2, 3, 4, 5, 6] ?
# 7+6+5+4
