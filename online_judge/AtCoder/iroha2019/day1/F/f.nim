import strutils, sequtils, tables

proc make(n: int): Table[int, int] =
  result = initTable[int, int]()
  var
    i = 2
    m = n
  while i * i <= m:
    if m mod i == 0:
      result[i] = 0
      while m mod i == 0:
        result[i] += 1
        m = m div i
    i += 1
  if m > 1:
    result[m] = 1

proc main() =
  var n, k: int
  (n, k) = stdin.readLine.strip.split.map(parseInt)

  if k == 1:
    if n == 1:
      echo "-1"
    else:
      echo n
    return
  var
    a = newSeq[int]()
    m = n
    i = 2
  while i * i <= m and a.len < k:
    while m mod i == 0 and a.len < k:
      a.add(i)
      m = m div i
    i += 1
  if a.len == k:
    a[^1] *= m
  elif m >= 2:
    a.add(m)
  var prod = 1
  for it in a:
    prod *= it
    doAssert(it >= 2)
  doAssert(prod == n)
  if a.len == k:
    echo a.mapIt($it).join(" ")
  else:
    echo "-1"
main()
