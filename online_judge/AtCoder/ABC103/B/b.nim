import strutils, sequtils

var
  s=stdin.readLine
  t=stdin.readLine

for i in 0..<s.len:
  var u=""
  for j in 0..<s.len: u&= $s[(i+j) mod s.len]
  if u==t:
    echo "Yes"
    quit 0

echo "No"