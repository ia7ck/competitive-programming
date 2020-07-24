import strutils, sequtils

proc valid(n: int): bool =
  var
    nn = n
    three = 0
    five = 0
    seven = 0
  while nn>0:
    let d = nn mod 10
    if d==3:
      three+=1
    elif d==5:
      five+=1
    elif d==7:
      seven+=1
    nn = nn div 10
  result = (three>=1 and five>=1 and seven>=1)


proc main() =
  let
    n = stdin.readLine.parseInt
    num = @[3, 5, 7]
  var
    cand = @[0]
    ans = 0
  while len(cand)>0:
    var next = newSeq[int]()
    for x in cand:
      for d in num:
        let y = x*10+d
        if y<=n:
          next.add(y)
          if valid(y): ans+=1
    cand = next
  echo ans

main()
