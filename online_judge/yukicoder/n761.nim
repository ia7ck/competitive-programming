import strutils, sequtils

proc f(a: seq[int64], turn: bool): bool =
  let n = a.len
  if a.all(proc(x: int64): bool = a[0]==x):
    result = true
  else:
    let s = a.foldl(a+b)
    var j = 0
    while j<n and a[j]*n<s:
      j+=1
    let
      res1 = f(a[0..<j], not turn)
      res2 = f(a[j..<n], not turn)
    result = not (res1 and res2)

proc main() =
  let
    n = stdin.readLine.parseInt
    a = stdin.readLine.split.map(parseBiggestInt)
  echo if f(a, true): "First" else: "Second"
main()
