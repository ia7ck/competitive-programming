import strutils, sequtils, algorithm, future

var
  n=stdin.readLine.parseInt
  a=stdin.readLine.split.map(parseInt)
  memo=newSeq[int](1 shl n)

fill(memo, -1) # algorithm
proc f(bit: int): int=
  if memo[bit]>=0:
    return memo[bit]
  elif bit==(1 shl n)-1:
    return 0
  else:
    for i in 0..<n:
      if (bit and (1 shl i))>0:
        continue
      for j in 0..<i:
        if (bit and (1 shl j))>0:        
          continue
        memo[bit]=max(memo[bit], (a[i] xor a[j])+f(bit xor (1 shl i) xor (1 shl j)))
    return memo[bit]

echo f(0)