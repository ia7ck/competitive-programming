#! /usr/bin/env gosh
(let ([a (read)] [b (read)])
  (display (if (< a b) 0 1)))
