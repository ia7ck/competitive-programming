import strutils, sequtils, algorithm

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let n, C = read().parseInt
  type P = tuple[s, t: int]
  var progs = newSeqWith(C, newSeq[P]())
  for i in 0..<n:
    let s, t, c = read().parseInt
    progs[c - 1].add((s, t))
  var cul = newSeq[int](100001)
  for c in 0..<C:
    if progs[c].len > 0:
      progs[c].sort(cmp)
      let ps = progs[c]
      cul[ps[0].s - 1] += 1
      cul[ps[0].t] -= 1
      for i in 1..<ps.len:
        if ps[i - 1].t == ps[i].s:
          # つなげる
          cul[ps[i - 1].t] += 1
          cul[ps[i].t] -= 1
        else:
          cul[ps[i].s - 1] += 1
          cul[ps[i].t] -= 1
  for i in 1..<cul.len:
    cul[i] = cul[i] + cul[i - 1]
  echo cul.max
main()
