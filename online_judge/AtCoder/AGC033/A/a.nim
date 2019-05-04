import strutils, sequtils, queues

proc main() =
  var h, w: int
  (h, w) = stdin.readLine.strip.split.map(parseInt)
  let a = (0..<h).mapIt(stdin.readLine.strip)

  type P = object
    i, j: int
  var
    q = initQueue[P]()
    d = newSeqWith(h, newSeqWith(w, 1000000000))
  for i in 0..<h:
    for j in 0..<w:
      if a[i][j] == '#':
        d[i][j] = 0
        q.enqueue(P(i: i, j: j))
  let
    dy = @[0, 0, -1, 1]
    dx = @[1, -1, 0, 0]
  while q.len > 0:
    let cur = q.dequeue
    for k in 0..<4:
      let
        ni = cur.i + dy[k]
        nj = cur.j + dx[k]
      if ni >= 0 and nj >= 0 and ni < h and nj < w:
        if d[cur.i][cur.j] + 1 < d[ni][nj]:
          d[ni][nj] = d[cur.i][cur.j] + 1
          q.enqueue(P(i: ni, j: nj))
  var ans = 0
  for i in 0..<h:
    for j in 0..<w:
      if ans < d[i][j]:
        ans = d[i][j]
  echo ans

main()
