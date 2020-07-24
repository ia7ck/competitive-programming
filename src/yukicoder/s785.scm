#! /usr/bin/env gosh

(define (g a)
  (square 
    (if (and (= (length a) 1)
             (string=? (car a) "NONE"))
      16
      (- 16 (length a)))))

(display (let f ([i 0] [res 1])
  (if (= i 3)
    res
    (f (+ i 1) (* res (g (string-split (read-line) ",")))))))
