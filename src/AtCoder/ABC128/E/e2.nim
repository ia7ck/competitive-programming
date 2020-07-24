import mersenne
var mt = newMersenneTwister(0)

type Node = ref object
  val: int64
  size: int
  sum: int64
  left, right: Node

proc newNode(val: int64): Node =
  let node = new Node
  node.val = val
  node.size = 1
  node.sum = val
  return node

type Pair = Node

proc makePair(left, right: Node): Pair =
  let pair = new Pair
  pair.left = left
  pair.right = right
  return pair

proc size(root: Node): int =
  return if root == nil: 0 else: root.size
proc sum(root: Node): int64 =
  return if root == nil: 0.int64 else: root.sum

proc update(node: Node): Node =
  node.size = 1 + size(node.left) + size(node.right)
  node.sum = node.val + sum(node.left) + sum(node.right)
  return node

proc merge(left, right: Node): Node =
  if left == nil:
    return right
  elif right == nil:
    return left
  if mt.getNum().int mod (size(left) + size(right)) < size(left):
    left.right = merge(left.right, right)
    return update(left)
  else:
    right.left = merge(left, right.left)
    return update(right)

proc split(root: Node, k: int): Pair =
  if root == nil:
    return makePair(root, root)
  if k <= size(root.left):
    let pair = split(root.left, k)
    root.left = pair.right
    return makePair(pair.left, update(root))
  else:
    let pair = split(root.right, k - size(root.left) - 1)
    root.right = pair.left
    return makePair(update(root), pair.right)

proc countLess(root: Node, x: int64): int =
  if root == nil:
    return 0
  if root.val < x:
    return size(root.left) + 1 + countLess(root.right, x)
  else:
    return countLess(root.left, x) # root.val==x の分はカウントしない

proc insert(root: Node, x: int64): Node =
  let
    k = root.countLess(x)
    pair = split(root, k)     # pair.left には x 未満しかない
  return merge(merge(pair.left, newNode(x)), pair.right)

proc erase(root: Node, x: int64): Node =
  let
    k = root.countLess(x)
    pair = split(root, k)
  if pair.right != nil:
    let pp = split(pair.right, 1) # pp.left は x のみ
    return merge(pair.left, pp.right)
  else: # x が存在しない
    return merge(pair.left, pair.right)

# 小さいほうからk番目 1-indexed
proc getKth(root: Node, k: int): Node =
  if root == nil or size(root) < k:
    return nil
  if size(root.left) + 1 == k:
    return root
  elif size(root.left) >= k:
    return getKth(root.left, k)
  else:
    return getKth(root.right, k - size(root.left) - 1)

# ---

import strutils, sequtils, algorithm, future

proc main() =
  var n, q: int
  (n, q) = stdin.readLine.strip.split.map(parseInt)
  type P = tuple[s, t, x: int]
  var items = newSeq[P]()
  for i in 0..<n:
    var s, t, x: int
    (s, t, x) = stdin.readLine.strip.split.map(parseInt)
    items.add((s, t, x))
  let d = (0..<q).mapIt(stdin.readLine.strip.parseInt)

  type E = tuple[t, val: int, add: bool]
  var events = newSeq[E]()
  for it in items:
    events.add((it.s - it.x, it.x, true))
    events.add((it.t - it.x, it.x, false))
  events.sort(proc(a, b: E): int =
    if a.t == b.t:
      if a.add == b.add:
        return cmp(a.t, b.t)
      else:
        return (if a.add: 1 else: -1)
    else:
      return cmp(a.t, b.t))
  let inf = 1000000000 + 7
  var
    mins = newSeqWith(n * 2, inf)
    root: Node = nil
  for i, ev in events:
    if ev.add:
      root = root.insert(ev.val)
    else:
      root = root.erase(ev.val)
    let mn = root.getKth(1)
    if mn != nil:
      mins[i] = mn.val.int
  for it in d:
    if it < events[0].t or events[^1].t < it:
      echo -1
      continue
    var
      ok = 0                  # events[ok].t <= it
      ng = n * 2
    while ng - ok > 1:
      let m = (ok + ng) div 2
      if events[m].t <= it:
        ok = m
      else:
        ng = m
    var ans = mins[ok]
    if ans == inf:
      ans = -1
    echo ans
main()
