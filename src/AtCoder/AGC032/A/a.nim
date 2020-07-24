import strutils, sequtils, algorithm

proc main() =
  var
    n = stdin.readLine.strip.parseInt
    a = stdin.readLine.strip.split.map(parseInt)

  var b = newSeq[int]()
  for i in 0..<n:
    var cand = newSeq[int]()
    for j in 0..<a.len:
      if j + 1 == a[j]:
        cand.add(j)
    if cand.len == 0:
      echo -1
      return
    b.add(cand[^1] + 1)
    a.delete(cand[^1], cand[^1])
  b.reverse
  for it in b:
    echo it

main() # ---
# 1
# 1 2
# 1 2 2
# 1 2 3 2
# 1 1 2 3 2
# 1 2 1 2 3 2
# 1 2 2 1 2 3 2
# 1 1 2 2 1 2 3 2
# 1 1 1 2 2 1 2 3 2
