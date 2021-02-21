#! /usr/bin/env gosh

(define (read-list n li)
  (if (= n 0)
    li
    (read-list (- n 1) (cons (read) li))))

(define (vector-add! vec i x)
  (vector-set! vec i (+ x (vector-ref vec i))))

(let* ([n (read)]
       [a (list->vector (reverse (read-list n (list))))]
       [freq (make-vector 100001 0)]
       [seen (make-vector 100001 0)]
       [ans n])
  (dotimes (_ n)
    (let ([b (read)])
      (vector-add! freq b 1)))
  (dotimes (i n)
    (let ([elm (vector-ref a i)])
      (vector-add! seen elm 1)
      (set! ans (min ans (quotient
                           (vector-ref freq elm)
                           (vector-ref seen elm))))
      (display ans)
      (newline))))
