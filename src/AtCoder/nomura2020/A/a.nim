import strutils

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let h1, m1, h2, m2, k = read().parseInt

  if h1 == h2:
    # 2 10 2 40 25
    # => 5
    let t = m2 - m1
    echo t - k
  else:
    let t = (60 - m1) + (h2 - h1 - 1) * 60 + m2
    echo t - k

main()
