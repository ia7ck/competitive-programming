import strutils, sequtils

proc main() =
  let
    s = stdin.readLine.strip
    n = s.len
  var
    i = 0
    ans = 0
    prev = "?"
  # a ab b a
  # a aa c ca c a b a ab a b c
  while i < n:
    if prev == s[i..i]:
      if i + 1 < n:
        prev = s[i..(i + 1)]
      else:
        break
      i += 2
      ans += 1
    else:
      prev = s[i..i]
      i += 1
      ans += 1
  echo ans
main()
