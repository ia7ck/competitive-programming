<?php

fscanf(STDIN, '%d', $N);
fscanf(STDIN, '%d', $K);
$xs = array_map('intval', explode(' ', fgets(STDIN)));

$ans = 0;
foreach ($xs as $x) {
    $ans += min($x, $K - $x) * 2;
}
echo $ans . "\n";
