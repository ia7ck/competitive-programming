#! /usr/bin/env gosh
(define (count-nz n)
  (if (= n 0)
    0
    (+ (if (= (modulo n 10) 0) 
         0 
         1)
       (count-nz (quotient n 10)))))

(define (hand n)
  (and (= (mod n 100) 0)
       (= (count-nz n) 1)))

(let* ([a (read)] [b (read)] [p (* a b)])
  (if (and (hand a) (hand b))
    (display (quotient p 10))
    (if (< (abs p) 100000000)
      (display p)
      (display "E"))))
