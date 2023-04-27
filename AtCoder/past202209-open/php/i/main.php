<?php

ob_start();

$T = (int) fgets(STDIN);
for ($t = 0; $t < $T; $t++) {
    [$N, $A, $M] = array_map('intval', explode(' ', fgets(STDIN)));
    $p = 0;
    $q = 0;
    for ($i = 1; $i <= $M; $i++) {
        $k = intdiv($A * $i + $M - 1, $M);
        $v = $k * $M - $A * $i;
        $p += $v;
        if ($i <= $N % $M) {
            $q += $v;
        }
    }
    $ans = intdiv($N, $M) * $p + $q;
    echo $ans, "\n";
}
