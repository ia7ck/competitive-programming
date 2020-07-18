import strutils

let read = iterator: string {.closure.} =
  while true:
    for s in stdin.readLine.split:
      yield s

proc main() =
  var
    s = read()
    n = s.len
  var
    cmb: int64 = 0
    kabu: int64 = 5
    a = newSeq[int64](n)
    b = newSeq[int64](n)
    score: int64 = 0
  for i in 0..<n:
    cmb += a[i]
    kabu += b[i]
    if s[i] == 'N':
      if kabu >= 1:
        score += 10 + (cmb div 10) * 1
        a[i + 2] += 1
        b[i + 7] += 1
        kabu -= 1
    elif s[i] == 'C':
      if kabu >= 3:
        score += 50 + (cmb div 10) * 5
        a[i + 4] += 1
        b[i + 9] += 3
        kabu -= 3
        if i + 1 < n:
          s[i + 1] = '-'
        if i + 2 < n:
          s[i + 2] = '-'
  echo score
main()
