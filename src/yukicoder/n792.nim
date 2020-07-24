import strutils, sequtils
proc main() =
  let
    n = stdin.readLine.parseInt
    a = (0..<(1 shl n)).mapIt(stdin.readLine.split.map(parseInt))
  var
    all_true = true
    all_false = true
    ans = "A="
  for i in 0..<(1 shl n):
    if a[i][n] == 1:
      all_false = false
      if ans.len > 2:
        ans &= "∨"
      ans &= "("
      for j in 0..<n:
        if j > 0:
          ans &= "∧"
        if a[i][j] == 0:
          ans &= "¬"
        ans &= "P_" & $(j+1)
      ans &= ")"
    else:
      all_true = false
  if all_true:
    ans = "A=⊤"
  elif all_false:
    ans = "A=⊥"
  echo ans
main()
