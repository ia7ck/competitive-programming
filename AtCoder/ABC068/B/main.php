<?php

function f(int $n): int
{
    $c = 0;
    while ($n % 2 == 0) {
        $n /= 2;
        $c += 1;
    }
    return $c;
}

fscanf(STDIN, '%d', $N);

$ans = 1;
for ($i = 1; $i <= $N; $i++) {
    $c = f($i);
    if (f($ans) < $c) {
        $ans = $i;
    }
}
echo $ans . "\n";
