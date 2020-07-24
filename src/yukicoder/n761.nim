import strutils, sequtils

proc f(a: seq[int64]): bool = # aを受け取った人は必勝？
  let n = a.len
  if a.all(proc(x: int64): bool = a[0]==x):
    result = true             # 残り全部取れるから勝ち
  else:
    let s = a.foldl(a+b)
    var j = 0
    while j<n and a[j]*n<s:
      j+=1
    let
      res1 = f(a[0..<j])
      res2 = f(a[j..<n])
    result = not (res1 and res2) # 相手が必勝&必勝だったら負け

proc main() =
  let
    n = stdin.readLine.parseInt
    a = stdin.readLine.split.map(parseBiggestInt)
  echo if f(a): "First" else: "Second"
main()
