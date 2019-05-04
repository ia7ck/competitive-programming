import strutils, sequtils, math

proc main() =
  var h, w, n: int
  (h, w, n) = stdin.readLine.strip.split.map(parseInt)
  var y, x: int
  (y, x) = stdin.readLine.strip.split.map(parseInt)
  let
    s = stdin.readLine.strip
    t = stdin.readLine.strip

  var
    win_l = newSeq[int](n)
    win_r = newSeq[int](n)
  win_l[n - 1] = 1
  win_r[n - 1] = w
  if s[n - 1] == 'L':
    win_l[n - 1] += 1
  if s[n - 1] == 'R':
    win_r[n - 1] -= 1
  for i in countdown(n - 2, 0):
    if t[i] == 'R': # Rを使えばwin_l[i+1]に行ける
      win_l[i] = max(1, win_l[i + 1] - 1)
    else:
      win_l[i] = win_l[i + 1]
    if t[i] == 'L':
      win_r[i] = min(w, win_r[i + 1] + 1)
    else:
      win_r[i] = win_r[i + 1]
    if s[i] == 'L':
      win_l[i] += 1
    if s[i] == 'R':
      win_r[i] -= 1
    if win_l[i] > win_r[i]:
      echo "NO"
      return

  var
    win_u = newSeq[int](n)
    win_d = newSeq[int](n)
  win_u[n - 1] = 1
  win_d[n - 1] = h
  if s[n - 1] == 'U':
    win_u[n - 1] += 1
  if s[n - 1] == 'D':
    win_d[n - 1] -= 1
  for i in countdown(n - 2, 0):
    if t[i] == 'D': # Dを使えばwin_u[i+1]に行ける
      win_u[i] = max(1, win_u[i + 1] - 1)
    else:
      win_u[i] = win_u[i + 1]
    if t[i] == 'U':
      win_d[i] = min(h, win_d[i + 1] + 1)
    else:
      win_d[i] = win_d[i + 1]
    if s[i] == 'U':
      win_u[i] += 1
    if s[i] == 'D':
      win_d[i] -= 1
    if win_u[i] > win_d[i]:
      echo "NO"
      return

  if win_l[0] <= x and x <= win_r[0] and win_u[0] <= y and y <= win_d[0]:
    echo "YES"
  else:
    echo "NO"

main()
