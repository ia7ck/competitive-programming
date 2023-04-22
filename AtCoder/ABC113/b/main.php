<?php

fscanf(STDIN, '%d', $N);
fscanf(STDIN, '%d %d', $T, $A);
$H = array_map('intval', explode(' ', fgets(STDIN)));

$ans = 0;
foreach ($H as $i => $h) {
    $b = $T - $H[$ans] * 0.006;
    $c = $T - $h * 0.006;
    if (abs($A - $b) > abs($A - $c)) {
        $ans = $i;
    }
}

echo $ans + 1 . "\n";
