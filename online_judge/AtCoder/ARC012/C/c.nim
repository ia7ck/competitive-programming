import strutils, sequtils

proc valid(a: seq[string]): bool =
  let n = a.len
  for i in 0..<n:
    for j in 0..<n:
      if a[i][j] == '.':
        continue
      for di in -1..1:
        for dj in -1..1:
          if di == 0 and dj == 0:
            continue
          var
            ni = i
            nj = j
            len = 0
          for k in 0..<5:
            if ni < 0 or nj < 0 or ni >= n or nj >= n:
              break
            if a[ni][nj] != a[i][j]:
              break
            len += 1
            ni += di
            nj += dj
          if len == 5:
            return false
  return true

proc main() =
  let n = 19
  var a = (0..<n).mapIt(stdin.readLine.strip)

  var
    bk_cnt = 0
    wt_cnt = 0
  for row in a:
    bk_cnt += row.filterIt(it == 'o').len
    wt_cnt += row.filterIt(it == 'x').len
  if bk_cnt != wt_cnt and bk_cnt != wt_cnt + 1:
    echo "NO"
    return
  if bk_cnt == 0 and wt_cnt == 0:
    echo "YES"
    return
  let last = if bk_cnt == wt_cnt: 'x' else: 'o'
  for i in 0..<n:
    for j in 0..<n:
      if a[i][j] == last:
        a[i][j] = '.'
        if valid(a):
          echo "YES"
          return
        a[i][j] = last
  echo "NO"
main()
