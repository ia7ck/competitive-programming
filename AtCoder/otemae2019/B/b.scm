#! /usr/bin/env gosh

(let* ([m (read)]
       [n (read)]
       [k (read)]
       [a (make-vector m 0)])
  (dotimes (_ n) 
    (let ([x (- (read) 1)])
      (vector-set! a x (+ (vector-ref a x) 1))))
  (display 
    (let center-loop ([c 0] [mx 0])
      (if (= c m) mx 
        (center-loop
          (+ c 1) 
          (max mx
               (let k-loop ([i 1] [s (vector-ref a c)])
                 (if (> i k) s
                   (k-loop
                     (+ i 1)
                     (+ s 
                        (if (or (and (>= (- c i) 0)
                                     (>= (vector-ref a (- c i)) 1))
                                (and (< (+ c i) m) 
                                     (>= (vector-ref a (+ c i)) 1)))
                          1
                          0)))))))))))
