import strutils, algorithm

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc solve(nn: int64): string =
  var
    n = nn
    s = ""
  while n > 0:
    var r = (n - 1) mod 26
    let c = (char)('a'.ord + r)
    s.add(c)
    n = (n - 1) div 26
  s.reverse
  return s

proc main() =
  let n = read().parseBiggestInt

  echo solve(n)
main()

proc sub() =
  for n in 1..100:
    echo solve(n)

# sub()
