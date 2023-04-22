<?php

fscanf(STDIN, '%d', $N);
$s = [];
for ($i = 0; $i < $N; $i++) {
    fscanf(STDIN, '%s', $x);
    $s[] = $x;
}
fscanf(STDIN, '%d', $M);
$t = [];
for ($i = 0; $i < $M; $i++) {
    fscanf(STDIN, '%s', $x);
    $t[] = $x;
}

$ans = 0;
foreach ($s as $x) {
    $plus = count(array_filter($s, function ($y) use ($x) {
        return $x == $y;
    }));
    $minus = count(array_filter($t, function ($y) use ($x) {
        return $x == $y;
    }));
    $ans = max($ans, $plus - $minus);
}

echo $ans . "\n";
