import strutils, sequtils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let
    h, w, n = read().parseInt
    sy, sx = read().parseInt
    s = read()
    t = read()

  var
    le = 1
    ri = w
    tp = 1
    bt = h
  for i in countdown(n - 1, 0):
    case t[i]:
      of 'L':
        ri = min(w, ri + 1)
      of 'R':
        le = max(1, le - 1)
      of 'U':
        bt = min(h, bt + 1)
      of 'D':
        tp = max(1, tp - 1)
      else: discard
    case s[i]:
      of 'L':
        le = le + 1
      of 'R':
        ri = ri - 1
      of 'U':
        tp = tp + 1
      of 'D':
        bt = bt - 1
      else: discard
    if le > ri or tp > bt:
      echo "NO"
      return
  if sx in le..ri and sy in tp..bt:
    echo "YES"
  else:
    echo "NO"
main()
