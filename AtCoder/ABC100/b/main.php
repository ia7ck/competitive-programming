<?php

fscanf(STDIN, '%d %d', $D, $N);

if ($N == 100) {
    $ans = ($N + 1) * pow(100, $D);
} else {
    $ans = $N * pow(100, $D);
}

echo $ans . "\n";
