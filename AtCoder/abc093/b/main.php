<?php

fscanf(STDIN, '%d %d %d', $A, $B, $K);

$xs = [];
for ($x = $A; $x < $A + $K && $x <= $B; $x++) {
    $xs[] = $x;
}
for ($x = $B; $x > $B - $K && $x >= $A; $x--) {
    $xs[] = $x;
}

$xs = array_unique($xs);
sort($xs);
foreach ($xs as $x) {
    echo $x . "\n";
}
