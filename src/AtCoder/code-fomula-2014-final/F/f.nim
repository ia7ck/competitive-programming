import strutils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  const
    n = 100
    w = 1500
  var pts = newSeq[(int, int)](n + 1)
  for k in 1..n:
    pts[k] = (-1, -1)
  var
    k = n
    cx = n
    cy = n
  while k > 0 and cx <= w and cy <= w:
    let kk = k
    var
      x = cx
      y = cy
    while k > 0 and x + k <= w:
      pts[k] = (x, y)
      x += k + k - 1
      k -= 1
    x = cx
    y = cy + kk + k
    while k > 0 and y + k <= w:
      pts[k] = (x, y)
      y += k + k - 1
      k -= 1
    cx += kk + k
    cy += kk + k
  for k in 1..n:
    let (x, y) = pts[k]
    echo x, " ", y
  for i in 1..n:
    for j in (i + 1)..n:
      let
        (x, y) = pts[i]
        (xx, yy) = pts[j]
      if (i + j) * (i + j) > (x - xx) * (x - xx) + (y - yy) * (y - yy):
        doAssert(false)
main()
