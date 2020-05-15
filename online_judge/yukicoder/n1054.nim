import strutils, sequtils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

type U = object
  root: seq[int]
  acc: seq[int64]
  val: seq[int64]
  nodes: seq[seq[int]]

proc init(n: int): U =
  return U(
    root: toSeq(0..(n - 1)),
    acc: newSeqWith(n, 0.int64),
    val: newSeqWith(n, 0.int64),
    nodes: toSeq(0..(n - 1)).mapIt(@[it]))

proc find(u: var U, i: int): int =
  if u.root[i] != i:
    u.root[i] = u.find(u.root[i])
  return u.root[i]

proc unite(u: var U, i0, j0: int) =
  var
    i = u.find(i0)
    j = u.find(j0)
  if i == j:
    return
  if u.nodes[i].len < u.nodes[j].len:
    swap(i, j)
  u.root[j] = i
  # echo "> ", i, " ", j, " ", u.nodes[j]
  let
    a = u.acc[j]
    b = u.acc[i]
  for k in u.nodes[j]:
    u.val[k] += a
    u.val[k] -= b
    u.acc[k] = 0
    u.nodes[i].add(k)
  u.nodes[j] = @[]

proc get(u: var U, i: int): int64 = u.acc[u.find(i)] + u.val[i]

proc add(u: var U, i: int, x: int) =
  u.acc[u.find(i)] += x

proc main() =
  let n, q = read().parseInt
  var u = init(n)
  for qq in 0..<q:
    let t, a, b = read().parseInt
    if t == 1:
      u.unite(a - 1, b - 1)
    elif t == 2:
      u.add(a - 1, b)
    else:
      echo u.get(a - 1)
    # echo u.acc
main()
