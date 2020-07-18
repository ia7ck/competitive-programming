import strutils, sequtils, algorithm, deques

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let
    n, k = read().parseInt
    a = newSeqWith(n, read().parseBiggestInt)

  const mo: int64 = 1000000007
  proc o(ans: int64) =
    echo (ans mod mo + mo) mod mo

  var
    b = a.filterIt(it >= 0)
    c = a.filterIt(it < 0)

  if n == k:
    var ans: int64 = 1
    for x in a:
      ans = ans * x mod mo
    o(ans)
    return

  if a.len == c.len:
    c.sort(cmp)
    var ans: int64 = 1
    if k mod 2 == 1:
      c.reverse
    for i in 0..<k:
      ans = ans * c[i] mod mo
    o(ans)
    return

  b.sort(cmp)
  b.reverse
  c.sort(cmp)
  var
    qb = initDeque[int64]()
    qc = initDeque[int64]()
  for x in b:
    qb.addLast(x)
  for x in c:
    qc.addLast(x)
  var
    m = 0
    ans: int64 = 1
  while m < k:
    if k mod 2 == 1 and m == k - 1:
      break
    if qb.len < 2:
      doAssert(qc.len >= 2)
      let
        x = qc.popFirst
        y = qc.popFirst
      ans = ans * (x * y mod mo) mod mo
      m += 2
    elif qc.len < 2:
      doAssert(qb.len >= 2)
      let
        u = qb.popFirst
        v = qb.popFirst
      ans = ans * (u * v mod mo) mod mo
      m += 2
    else:
      let
        x = qc.popFirst
        y = qc.popFirst
        u = qb.popFirst
        v = qb.popFirst
      if x * y >= u * v:
        ans = ans * (x * y mod mo) mod mo
        qb.addFirst(v)
        qb.addFirst(u)
      else:
        ans = ans * (u * v mod mo) mod mo
        qc.addFirst(y)
        qc.addFirst(x)
      m += 2
  if m < k:
    doAssert(qb.len >= 1)
    let u = qb.popFirst
    ans = ans * u mod mo
  o(ans)
main()
