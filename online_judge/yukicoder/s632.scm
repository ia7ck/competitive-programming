#! /usr/bin/env gosh
(define (kado s)
  (let* ([l (string-split s " ")]
         [a (string->number (car l))]
         [b (string->number (car (cdr l)))]
         [c (string->number (car (cdr (cdr l))))])
      (and (not (= a c))
           (or (and (< a b)
                    (< c b))
               (and (> a b)
                    (> c b))))))
(let ([s (read-line)])
  (display (if (kado (regexp-replace #/\?/ s "1"))
             "1"
             ""))
  (display (if (kado (regexp-replace #/\?/ s "4"))
             "4"
             "")))
