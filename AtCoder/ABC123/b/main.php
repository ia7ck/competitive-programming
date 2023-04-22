<?php

function f(int $x): int
{
    return intdiv($x + 10 - 1, 10) * 10;
}

fscanf(STDIN, '%d', $A);
fscanf(STDIN, '%d', $B);
fscanf(STDIN, '%d', $C);
fscanf(STDIN, '%d', $D);
fscanf(STDIN, '%d', $E);

$ans = min([
    $A + f($B) + f($C) + f($D) + f($E),
    f($A) + $B + f($C) + f($D) + f($E),
    f($A) + f($B) + $C + f($D) + f($E),
    f($A) + f($B) + f($C) + $D + f($E),
    f($A) + f($B) + f($C) + f($D) + $E,
]);

echo $ans . "\n";
