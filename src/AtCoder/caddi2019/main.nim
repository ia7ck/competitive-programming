import strutils, sequtils, times, algorithm, mersenne

const
  L = 1000
  N = 1000
  M = 100000
  R = 200

type Ball = object
  r: int
  p: int64
  i: int

type Special = object
  i: int
  j: int
  d: int
  p: int64

type Point = object
  x: int
  y: int
  z: int
  r: int

var
  balls = newSeq[Ball](N)
  specials = newSeq[Special](M)
  ans = newSeq[Point](N)
  mt = newMersenneTwister(123)
  used = newSeq[bool](N)
  dat = newSeqWith(R + 1, newSeq[int]()) # dat[r] := b.r==rになってるballのidたち
  foo = newSeqWith(N, newSeq[int]()) # foo[i] := s.i==iになってるsたちのidたち
  bar = newSeqWith(N, newSeq[int]()) # bar[i] := s.j==iになってるsたちのidたち

proc sq(n: int): int = n*n

proc dist(a, b: Point): int =
  return sq(a.x - b.x) + sq(a.y - b.y) + sq(a.z - b.z)

proc check() =
  for i in 0..<N:
    if used[i]:
      let
        a = ans[i]
        r = a.r
      assert(a.x - r >= 0)
      assert(a.y - r >= 0)
      assert(a.z - r >= 0)
      assert(a.x + r <= L)
      assert(a.y + r <= L)
      assert(a.z + r <= L)
  for i in 0..<ans.len:
    for j in 0..<i:
      if used[i] and used[j]:
        let
          ai = ans[i]
          aj = ans[j]
          d = dist(ai, aj)
        assert(d >= (ai.r - aj.r) * (ai.r - aj.r))

# [l, r] in [0, L]
proc rand(mt: var MersenneTwister, mn, mx: int): int =
  if mx - mn < 0:
    return 0
  else:
    return (mt.getNum() mod (mx - mn + 1)) + mn

proc init(): seq[Point] =
  fill(ans, Point(x: -1, y: -1, z: -1, r: -1))
  balls.sort(proc(a, b: Ball): int = -system.cmp(a.r, b.r))
  var pts = newSeq[Point]()
  for i, ball in balls[0..<N]:
    for t in 0..<(i + 1) * 100:
      let
        r = ball.r
        x = mt.rand(r, L - r)
        y = mt.rand(r, L - r)
        z = mt.rand(r, L - r)
      var can = true
      for pt in pts:
        if sq(pt.x - x) + sq(pt.y - y) + sq(pt.z - z) < sq(r + pt.r):
          can = false
          break
      if can:
        pts.add(Point(x: x, y: y, z: z, r: r))
        used[ball.i] = true
        ans[ball.i] = pts[^1]
        break
  balls.sort(proc(a, b: Ball): int = system.cmp(a.i, b.i))
  return pts

proc calcScore(): int64 =
  for i in 0..<N:
    if used[i]:
      result += balls[i].p
  for s in specials:
    if used[s.i] and used[s.j]:
      if dist(ans[s.i], ans[s.j]) <= sq(s.d):
        result += s.p

# i, jがスコアに寄与するぶんを返す
proc calcPartial(i, j: int): int64 =
  if used[i]:
    result += balls[i].p
    for k in foo[i]:
      if used[specials[k].j]:
        if dist(ans[i], ans[specials[k].j]) <= sq(specials[k].d):
          result += specials[k].p
    for k in bar[i]:
      if used[specials[k].i]:
        if dist(ans[i], ans[specials[k].i]) <= sq(specials[k].d):
          result += specials[k].p
  if used[j]:
    result += balls[j].p
    for k in foo[j]:
      if used[specials[k].j]:
        if dist(ans[j], ans[specials[k].j]) <= sq(specials[k].d):
          result += specials[k].p
    for k in bar[j]:
      if used[specials[k].i]:
        if dist(ans[j], ans[specials[k].i]) <= sq(specials[k].d):
          result += specials[k].p

proc main() =
  # ---
  discard stdin.readLine.split.map(parseInt)
  for i in 0..<N:
    balls[i].i = i
    (balls[i].r, balls[i].p) = stdin.readLine.split.map(parseInt)
  for i in 0..<M:
    (specials[i].i, specials[i].j, specials[i].d,
        specials[i].p) = stdin.readLine.split.map(parseInt)
    specials[i].i -= 1
    specials[i].j -= 1
  # ---

  var pts = init()
  for ball in balls:
    dat[ball.r].add(ball.i)
  for i, s in specials:
    foo[s.i].add(i)
    bar[s.j].add(i)
  for cc in 0..<100000:
    let r = mt.rand(1, R)
    if dat[r].len <= 1:
      continue
    let
      j2 = mt.rand(1, dat[r].len - 1)
      i2 = mt.rand(0, j2 - 1) # i < j
      j = dat[r][j2]
      i = dat[r][i2]
    if (not used[i]) and (not used[j]):
      continue
    let pre = calcPartial(i, j)
    swap(ans[i], ans[j])
    swap(used[i], used[j])
    let nex = calcPartial(i, j)
    if nex < pre:
      swap(ans[i], ans[j])
      swap(used[i], used[j])

  # ---
  check()
  # for i in 0..<N:
  #   if used[i]:
  #     echo ans[i].x, " ", ans[i].y, " ", ans[i].z
  #   else:
  #     echo -1, " ", -1, " ", -1
  # ---

  echo pts.len
  echo calcScore()
main()
