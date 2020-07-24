import strutils, sequtils, algorithm

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let
    n, k, c = read().parseInt
    s = read()

  var
    # du[i]: i = 0, 1, ... 日目までに働ける最大の日数
    du = newSeq[int](n)
    last = int.low
  for i in 0..<n:
    if i > 0:
      du[i] = du[i - 1]
    if last + c < i and s[i] == 'o':
      du[i] += 1
      last = i
  # df[i]: i = 0, 1, ... 日目から働ける最大の日数
  var df = newSeq[int](n)
  for i in countdown(n - 1, 0):
    if i + 1 < n:
      df[i] = df[i + 1]
    if s[i] == 'o':
      df[i] = max(df[i], 1) # ...
      let j = i + c + 1
      if j < n:
        df[i] = max(df[i], df[j] + 1)
  # left[i]: j < i, s[j] = 'o' になってる最大の j
  var left = newSeq[int](n)
  last = -1
  for i in 0..<n:
    left[i] = last
    if s[i] == 'o':
      last = i
  # right[i]: i < j, s[j] = 'o' になってる最大の j
  var right = newSeq[int](n)
  last = n
  for i in countdown(n - 1, 0):
    right[i] = last
    if s[i] == 'o':
      last = i
  # ok[i]: i = 0, 1, ..., N - 1 日目は働かなくてもよいか
  var ok = newSeq[bool](n)
  for i in 0..<n:
    if s[i] == 'x':
      ok[i] = true
      continue
    var w = 0
    let j = left[i]
    if j >= 0:
      w += du[j]
      # let o = max(i + 1, j + c + 1)
      let o = max(i + 1, du.lowerBound(du[j]) + c + 1)
      if o < n:
        w += df[o]
    elif i + 1 < n:
      w += df[i + 1]
    # echo i, " ", j, " ", du.lowerBound(j), " ", w
    if w >= k:
      ok [i] = true
  # echo du
  # echo df
  for i in 0..<n:
    if not ok[i]:
      echo i + 1
main()
