import strutils, sequtils

const mo: int = 1000000000 + 7
proc add(a: var int, b: int) =
  a = (a + b) mod mo

proc main() =
  let s = stdin.readLine.strip

  const M = 13
  var dp = newSeq[int](M)
  dp[0] = 1
  for i in 0..<s.len:
    var nxt = newSeq[int](M)
    if isDigit(s[i]):
      let d = s[i].ord - '0'.ord
      for j in 0..<M:
        add(nxt[(j * 10 + d) mod M], dp[j])
    else:
      for j in 0..<M:
        for d in 0..<10:
          add(nxt[(j * 10 + d) mod M], dp[j])
    dp.swap(nxt)
  echo dp[5]
main()
