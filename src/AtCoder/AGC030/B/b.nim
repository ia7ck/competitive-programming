import strutils, sequtils, queues, algorithm

proc chmax(a: var int64, b: int64): bool =
  result = a<b
  if result:
    a = b

proc main() =
  let
    ln = stdin.readLine.split.map(parseInt)
    (ll, n) = (ln[0].int64, ln[1])
    xs = (0..<n).mapIt(stdin.readLine.parseBiggestInt)
  var dp = newSeqWith(n, newSeqWith(n, newSeq[int64](2)))
  type T = tuple[i, j, side: int] # [i, j]が残っていて、つぎにsideの端の木を燃やす; 0->左、1->右
  var q: Queue[T] = initQueue[T]()
  dp[0][n-1][0] = xs[0]
  dp[0][n-1][1] = ll-xs[n-1]
  q.add((i: 0, j: n-1, side: 0))
  q.add((i: 0, j: n-1, side: 1))
  while q.len>0:
    let
      cur = q.dequeue
      (i, j, s) = (cur.i, cur.j, cur.side)
      val = dp[i][j][s]
    if i+1<=j: # 2本以上残ってる？
      if s==0: # いま左
        if chmax(dp[i+1][j][0], val+(xs[i+1]-xs[i])): # つぎ左
          q.add((i: i+1, j: j, side: 0))
        if chmax(dp[i+1][j][1], val+(ll-(xs[j]-xs[i]))): # つぎ右
          q.add((i: i+1, j: j, side: 1))
      else: # いま右
        if chmax(dp[i][j-1][0], val+(ll-(xs[j]-xs[i]))): # つぎ左
          q.add((i: i, j: j-1, side: 0))
        if chmax(dp[i][j-1][1], val+(xs[j]-xs[j-1])): # つぎ右
          q.add((i: i, j: j-1, side: 1))
  var mx = 0'i64
  for i in 0..<n:
    let _ = chmax(mx, dp[i][i].max)
  echo mx
main()
