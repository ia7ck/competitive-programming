import strutils, sequtils, mersenne

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

# max(left) <= min(right) じゃないとダメ?
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

# k 個と size(root)-k 個に分ける
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

proc getKth(root: Node, k: int): Node =
  if root == nil or size(root) < k:
    return nil
  if size(root.left) + 1 == k:
    return root
  elif size(root.left) >= k:
    return getKth(root.left, k)
  else:
    return getKth(root.right, k - size(root.left) - 1)

proc main() =
  let
    nk = stdin.readLine.strip.split.map(parseInt)
    (n, k) = (nk[0], nk[1])
    a = stdin.readLine.strip.split.map(parseBiggestInt)

  var
    root: Node = nil
    ans = int64.high
  for i in 0..<n:
    root = root.insert(a[i])
    if i >= k:
      root = root.erase(a[i - k])
    if i + 1 >= k:
      let
        kth = root.getKth((k + 1) div 2)
        pair = root.split((k + 1) div 2)
      var cost: int64 = 0
      cost += kth.val * size(pair.left) - sum(pair.left)
      cost += sum(pair.right) - kth.val * size(pair.right)
      if cost < ans:
        ans = cost
      root = merge(pair.left, pair.right)
  echo ans

main()
