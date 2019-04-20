import strutils, sequtils, math, queues

proc main() =
  var
    nk = stdin.readLine.strip.split.map(parseInt)
    (n, k) = (nk[0], nk[1])
    a = stdin.readLine.strip.split.map(parseInt)
    str = (0..<n).mapIt(stdin.readLine.strip)

  if k == n:
    echo ""
    return
  var
    mnLen = 100000
    target = newSeq[bool](n)
  for i in 0..<k:
    a[i] -= 1
    target[a[i]] = true
    mnLen = min(mnLen, str[a[i]].len)
  type Page = object
    name: string
    index: int
  var q = initQueue[Page]()
  for i in 0..<n:
    q.enqueue(Page(name: str[i], index: i))
  var key = ""
  while key.len < mnLen:
    var ok = true
    for i in a:
      if str[a[0]][key.len] != str[i][key.len]:
        ok = false
        break
    if not ok:
      echo (-1)
      return
    key &= $str[a[0]][key.len]
    var nxt = initQueue[Page]()
    while q.len > 0:
      let cur = q.dequeue
      if cur.name.len < key.len:
        if target[cur.index]:
          echo (-1)
          return
      elif cur.name[key.len - 1] == key[^1]:
        nxt.enqueue(cur)
      elif target[cur.index]:
        echo (-1)
        return
    if nxt.len == k:
      echo key
      return
    else:
      q.swap(nxt)
  echo (-1)

main()
