import strutils, deques, sequtils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let
    h, w = read().parseInt
    a = newSeqWith(h, read())
  var
    q = initDeque[(int, int, int)]()
    seen = newSeqWith(h, newSeqWith(w, newSeq[bool](3)))
  for i in 0..<h:
    for j in 0..<w:
      if a[i][j] == 's':
        q.addLast((i, j, 0))
        seen[i][j][0] = true
  let
    dy = @[-1, 0, 0, 1]
    dx = @[0, -1, 1, 0]
  while q.len > 0:
    let (i, j, t) = q.popFirst
    if a[i][j] == 'g':
      echo "YES"
      return
    for k in 0..<4:
      let
        ni = i + dy[k]
        nj = j + dx[k]
      if 0 <= ni and ni < h and 0 <= nj and nj < w:
        if a[ni][nj] != '#':
          if not seen[ni][nj][t]:
            q.addLast((ni, nj, t))
            seen[ni][nj][t] = true
        else:
          if t < 2 and (not seen[ni][nj][t + 1]):
            q.addLast((ni, nj, t + 1))
            seen[ni][nj][t + 1] = true
  echo "NO"
main()
