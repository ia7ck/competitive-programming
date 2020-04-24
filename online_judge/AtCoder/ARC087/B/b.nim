import strutils, sequtils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let
    s = read()
    gx, gy = read().parseInt
  let D = s.len + 5
  # (D, D) --> (gx + D, gy + D)
  var
    f = 0
    t = newSeq[int]()
  for c in s:
    if c == 'F':
      f += 1
    else:
      t.add(f)
      t.add(-1)
      f = 0
  t.add(f)
  var
    dpX = newSeq[seq[bool]](t.len + 1)
    dpY = newSeq[seq[bool]](t.len + 1)
  for i in 0..t.len:
    dpX[i] = newSeq[bool](D * 2)
    dpY[i] = newSeq[bool](D * 2)
  dpX[0][0 + D] = true
  dpY[0][0 + D] = true
  proc update(p: var bool, q: bool) =
    p = p or q
  var o = 0
  for i in 0..<t.len:
    let d = t[i]
    if d == -1:
      o += 1
    for a in 0..<(D * 2):
      if d == -1:
        update(dpX[i + 1][a], dpX[i][a])
        update(dpY[i + 1][a], dpY[i][a])
      else:
        if o mod 2 == 0:
          if o == 0:
            if a + d < D * 2:
              update(dpX[i + 1][a + d], dpX[i][a])
          else:
            if a + d < D * 2:
              update(dpX[i + 1][a + d], dpX[i][a])
            if a - d >= 0:
              update(dpX[i + 1][a - d], dpX[i][a])
          update(dpY[i + 1][a], dpY[i][a])
        else:
          # echo d
          if a + d < D * 2:
            update(dpY[i + 1][a + d], dpY[i][a])
          if a - d >= 0:
            update(dpY[i + 1][a - d], dpY[i][a])
          update(dpX[i + 1][a], dpX[i][a])
  # echo dpX[1][1 + D]
  # for x in (-s.len)..s.len:
    # echo x, " ", dpX[2][x + D]
  # echo t
  let ans = dpX[t.len][gx + D] and dpY[t.len][gy + D]
  if ans:
    echo "Yes"
  else:
    echo "No"
main()
