import strutils, sequtils, algorithm

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc solve(i: int, cur: char, s: string, ans: var seq[string]) =
  if i == 0:
    ans.add(s)
  elif cur == '!':
    solve(i - 1, 'a', s & 'a', ans)
  else:
    for c in ('a'.ord)..(cur.ord):
      solve(i - 1, cur, s & c.char, ans)
    let nxt = (cur.ord + 1).char
    solve(i - 1, nxt, s & nxt, ans)

proc main() =
  let n = read().parseInt

  var ans = newSeq[string]()
  solve(n, '!', "", ans)
  echo ans.sortedByIt(it).join("\n")
main()
