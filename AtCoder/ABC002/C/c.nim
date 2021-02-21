import strutils, sequtils

proc main() =
  let
    ps = stdin.readLine.split.map(parseInt)
    ux = ps[2]-ps[0]
    vx = ps[4]-ps[0]
    uy = ps[3]-ps[1]
    vy = ps[5]-ps[1]
  echo abs(ux*vy-uy*vx) / 2

main()
