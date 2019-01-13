import strutils, sequtils, queues
proc main() =
  let
    hw = stdin.readLine.split.map(parseInt)
    (h, w) = (hw[0], hw[1])
    c = (0..<h).mapIt(stdin.readLine)
    dy = @[1, 0, -1, 0]
    dx = @[0, 1, 0, -1]
  type T = object
    i, j: int
    col: char
  var
    seen = newSeqWith(h, newSeqWith(w, false))
    ans = 0'i64
  for i in 0..<h:
    for j in 0..<w:
      if c[i][j]=='#' and not seen[i][j]:
        var
          q = initQueue[T]()
          (nw, nb) = (0'i64, 1'i64)
        seen[i][j] = true
        q.add(T(i: i, j: j, col: '#'))
        while q.len>0:
          let cur = q.dequeue
          for k in 0..<4:
            let
              ni = cur.i+dy[k]
              nj = cur.j+dx[k]
            if 0<=ni and ni<h and 0<=nj and nj<w:
              if not seen[ni][nj] and c[ni][nj]!=cur.col:
                if c[ni][nj]=='.':
                  nw+=1
                else:
                  nb+=1
                seen[ni][nj] = true
                q.add(T(i: ni, j: nj, col: c[ni][nj]))
        ans+=nw*nb
  echo ans

main()
