import strutils, sequtils, tables
proc main() =
  let
    n = stdin.readLine.parseInt
    strs = (0..<3).mapIt(stdin.readLine)
  var ans = 0
  for i in 0..<n:
    var tab = newTable[char, int]()
    for j in 0..<3:
      if tab.hasKey(strs[j][i]):
        tab[strs[j][i]]+=1
      else:
        tab[strs[j][i]] = 1
    if tab.len==1:
      discard
    elif tab.len==2:
      ans+=1
    else:
      ans+=2
  echo ans
main()
