<?php

fscanf(STDIN, '%d %d %d', $N, $M, $X);
$A = array_map('intval', explode(' ', fgets(STDIN)));

$lt = array_filter($A, function ($a) use ($X) {
    return $a < $X;
});
$gt = array_filter($A, function ($a) use ($X) {
    return $a > $X;
});

$ans = min(count($lt), count($gt));
echo $ans . "\n";
