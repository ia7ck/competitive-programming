import strutils, sequtils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  let
    h, w = read().parseInt
    s = newSeqWith(h, read())
    dy = @[-1, 0, 0, 1]
    dx = @[0, -1, 1, 0]
  var
    seen = newSeqWith(h, newSeq[bool](w))
    ans = 0
  proc dfs(i, j: int, bk, wh: var int) =
    seen[i][j] = true
    if s[i][j] == '#':
      bk += 1
    else:
      wh += 1
    for k in 0..<4:
      let
        ni = i + dy[k]
        nj = j + dx[k]
      if ni < 0 or ni >= h or nj < 0 or nj >= w:
        continue
      if s[ni][nj] != s[i][j] and (not seen[ni][nj]):
        dfs(ni, nj, bk, wh)
  for i in 0..<h:
    for j in 0..<w:
      var
        bk = 0
        wh = 0
      dfs(i, j, bk, wh)
      ans += bk * wh
  echo ans
main()
