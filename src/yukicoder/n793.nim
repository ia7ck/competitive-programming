import strutils, sequtils, tables

proc po(b, e, mo: int64): int64 =
  if e == 0:
    result = 1
  elif e == 1:
    result = b
  elif e mod 2 == 0:
    result = po(b*b mod mo, e div 2, mo)
  else:
    result = b * po(b, e - 1, mo) mod mo

proc main() =
  let
    n = stdin.readLine.parseInt
    mo = 1_000_000_000 + 7
  var ans = po(10, n, mo) - 1
  ans = ans * po(9, mo-2, mo) mod mo
  ans = ans * 3 mod mo
  ans = (ans + po(10, n, mo)) mod mo
  echo ans
main()
