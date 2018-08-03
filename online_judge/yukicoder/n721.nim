import strutils, sequtils, times

var
  t=parse(stdin.readLine, "yyyy/MM/dd")

echo format(t+2.days, "yyyy/MM/dd")