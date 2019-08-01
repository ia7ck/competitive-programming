#! /usr/bin/env gosh

(let ([l (read)] [r (read)] [n (read)])
  (dotimes (i n)
    (display (- (div (- r i) n) 
                (div (- l i 1) n)))
    (newline)))
