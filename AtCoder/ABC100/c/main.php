<?php

fscanf(STDIN, '%d', $N);
$a = array_map('intval', explode(' ', fgets(STDIN)));

$ans = 0;
foreach ($a as $x) {
    while ($x % 2 == 0) {
        $ans += 1;
        $x /= 2;
    }
}

echo $ans . "\n";
