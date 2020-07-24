import strutils, sequtils, queues, algorithm
proc main() =
  var h, w: int
  (h, w) = stdin.readLine.strip.split.map(parseInt)
  var c = (0..<h).mapIt(stdin.readLine.strip)

  type P = tuple[y, x: int]
  const
    inf = 1000000000
    dy = @[-1, 0, 0, 1]
    dx = @[0, -1, 1, 0]
  var ans = 1
  for si in 0..<h:
    for sj in 0..<w:
      if c[si][sj] == '#': continue
      var
        q = initQueue[P]()
        d = newSeqWith(h, newSeq[int](w))
      for y in 0..<h:
        fill(d[y], inf)
      d[si][sj] = 0
      q.enqueue((si, sj))
      while q.len > 0:
        let p = q.dequeue
        for k in 0..<4:
          let
            ny = p.y + dy[k]
            nx = p.x + dx[k]
          if ny < 0 or nx < 0 or ny >= h or nx >= w: continue
          if c[ny][nx] == '#': continue
          if d[ny][nx] == inf:
            d[ny][nx] = d[p.y][p.x] + 1
            q.enqueue((ny, nx))
      for gi in 0..<h:
        for gj in 0..<w:
          if c[gi][gj] == '#': continue
          ans = max(ans, d[gi][gj])
  echo ans
main()
