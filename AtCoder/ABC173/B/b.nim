import strutils, sequtils, tables

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let
    n = read().parseInt
    ss = newSeqWith(n, read())

  var freq = ss.toCountTable
  for s in @["AC", "WA", "TLE", "RE"]:
    echo s, " x ", freq[s]
main()
