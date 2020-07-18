import strutils, sequtils, algorithm, future

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let n, c = read().parseInt
  type P = tuple[s, t, p: int]
  var progs = newSeq[P]()
  for i in 0..<n:
    let s, t, p = read().parseInt
    progs.add((s, t, p - 1))

  progs.sort((x, y) => cmp(x.s, y.s))
  var recs = newSeq[P]()
  for p in progs:
    var reuse = false
    for r in recs.mitems:
      if r.t < p.s or (r.p == p.p and r.t <= p.s):
        r = p
        reuse = true
        break
    if not reuse:
      recs.add(p)
  echo recs.len
main()
