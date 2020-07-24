import strutils, math

let read = iterator: string {.closure.} =
  for s in stdin.readAll.split:
    yield s

proc main() =
  let a, b, h, m = read().parseFloat

  let
    x = (h * 60 + m) / 12 / 60 * 360
    y = m / 60 * 360
    z = min(abs(x - y), 360 - abs(x - y))
    c = a * a + b * b - 2 * a * b * cos(degToRad(z))
  echo sqrt(c).formatFloat
main()
