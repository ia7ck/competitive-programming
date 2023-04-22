<?php

function f(string $s): bool
{
    str_replace(['A', 'C', 'G', 'T'], '', $s, $count);
    return strlen($s) == $count;
}

$S = fgets(STDIN);

$ans = 0;
for ($l = 0; $l < strlen($S); $l++) {
    for ($r = $l; $r < strlen($S); $r++) {
        if (f(substr($S, $l, $r - $l + 1))) {
            $ans = max($ans, $r - $l + 1);
        }
    }
}
echo $ans . "\n";
