import strutils, sequtils

proc main() =
  let s = stdin.readLine.strip
  var
    st = newSeq[char]()
    ans = 0
  for c in s:
    if st.len == 0:
      st.add(c)
    else:
      if c == st[st.len - 1]:
        st.add(c)
      else:
        let _ = st.pop
        ans += 2
  echo ans
main()

