#! /usr/bin/env gosh

(define (powmod a x m)
  (if (= x 0)
    1
    (if (= (remainder x 2) 0)
      (powmod (remainder (* a a) m) (quotient x 2) m)
      (remainder (* a (powmod a (- x 1) m)) m))))

(define (inv a m)
  (powmod a (- m 2) m))

(define mo 1000000007)

(define (binom n r acc)
  (if (= r 0)
    acc
    (binom n 
           (- r 1) 
           (remainder
             (* acc
                (- n (- r 1)) 
                (inv r mo)) mo))))

(let* ([n (read)] [k (read)])
  (display
    (if (> n k)
      (binom (- (+ n k) 1) k 1)
      (binom n (remainder k n) 1)))
  (newline))
