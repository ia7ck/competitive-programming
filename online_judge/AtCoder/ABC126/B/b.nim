import strutils, sequtils

proc is_month(s: string): bool =
  let m = s.parseInt
  return 1 <= m and m <= 12

proc main() =
  let s = stdin.readLine.strip

  let
    res1 = is_month(s[0..1])
    res2 = is_month(s[2..3])
  if res1 and res2:
    echo "AMBIGUOUS"
  elif res1:
    echo "MMYY"
  elif res2:
    echo "YYMM"
  else:
    echo "NA"

main()
