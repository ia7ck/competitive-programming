#! /usr/bin/env gosh

(let* ([n (read)]
      [a (let input ([i 0] [li (list)])
           (if (= i n)
             li
             (input (+ i 1) (cons (read) li))))])
  (let ([cost (let solve ([i 0] [li a] [tgt n] [res 0])
    (if (= i n)
      res
      (let* ([found (= (car li) tgt)])
        (solve (+ i 1)
               (cdr li)
               (- tgt (if found 1 0))
               (+ res (if found 0 1))))))])
    (display cost)))
