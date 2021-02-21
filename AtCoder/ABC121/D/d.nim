import strutils, sequtils

proc f(a: int64): int64 =
  for i in 1..a:
    result = result xor i

proc g(a: int64): int64 =
  case a mod 4
  of 0:
    result = a
  of 1:
    result = 1
  of 2:
    result = a + 1
  of 3:
    result = 0
  else:
    doAssert(false)

proc main() =
  let
    ab = stdin.readLine.strip.split.map(parseBiggestInt)
    (a, b) = (ab[0], ab[1])
  var ans = g(b)
  if a > 0:
    ans = ans xor g(a - 1)
  echo ans

main()
