<?php

fscanf(STDIN, '%d %d', $A, $B);

$ans = 0;
for ($x = $A; $x <= $B; $x++) {
    if ("$x" == strrev("$x")) {
        $ans += 1;
    }
}

echo $ans . "\n";
