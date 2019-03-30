import strutils, sequtils

proc main() =
  var
    n = stdin.readLine.strip.parseInt
    odd = false
  if n == 3:
    echo 2
    echo 1, " ", 3
    echo 2, " ", 3
    return
  if n mod 2 == 1:
    odd = true
    n -= 1
  type Edge = object
    frm: int
    to: int
  var edges = newSeq[Edge]()
  edges.add(Edge(frm: 1, to: 2))
  edges.add(Edge(frm: 1, to: n - 1))
  edges.add(Edge(frm: 1, to: n div 2))
  edges.add(Edge(frm: 1, to: n div 2 + 1))
  edges.add(Edge(frm: n, to: 2))
  edges.add(Edge(frm: n, to: n - 1))
  edges.add(Edge(frm: n, to: n div 2))
  edges.add(Edge(frm: n, to: n div 2 + 1))

  for i in 2..<(n div 2):
    edges.add(Edge(frm: i, to: i + 1))
    edges.add(Edge(frm: i, to: n - i))
  for i in countdown(n - 1, n div 2 + 2):
    edges.add(Edge(frm: i, to: i - 1))
    edges.add(Edge(frm: i, to: n - i + 2))

  if odd:
    for i in 1..n:
      edges.add(Edge(frm: i, to: n + 1))
  var
    seen = newSeqWith(n + 2, newSeq[bool](n + 2))
    ans = newSeq[Edge]()
  for e in edges:
    if e.frm != e.to:
      if not seen[e.frm][e.to] and not seen[e.to][e.frm]:
        ans.add(e)
        seen[e.frm][e.to] = true
        seen[e.to][e.frm] = true
  echo ans.len
  for a in ans:
    echo a.frm, " ", a.to
main()
