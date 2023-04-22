<?php

fscanf(STDIN, '%d', $N);
fscanf(STDIN, '%d %d', $D, $X);

$ans = $X;
for ($i = 0; $i < $N; $i++) {
    fscanf(STDIN, '%d', $a);
    $ans += intdiv($D + $a - 1, $a);
}
echo $ans . "\n";
