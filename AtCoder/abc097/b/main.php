<?php

fscanf(STDIN, '%d', $X);

$y = 1;
for ($b = 2; $b <= $X; $b++) {
    for ($p = 2; $b ** $p <= $X; $p++) {
        $y = max($y, $b ** $p);
    }
}

echo $y . "\n";
