import strutils, sequtils

var n=stdin.readLine.parseInt

echo((0..<n).mapIt(stdin.readLine).join)