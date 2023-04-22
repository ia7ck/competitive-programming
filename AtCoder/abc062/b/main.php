<?php

fscanf(STDIN, '%d %d', $H, $W);
$a = [];
for ($i = 0; $i < $H; $i++) {
    fscanf(STDIN, '%s', $s);
    $a[] = $s;
}

$b = [
    str_repeat('#', $W + 2),
    ... array_map(function ($s) {
        return "#$s#";
    }, $a),
    str_repeat('#', $W + 2),
];

foreach ($b as $s) {
    echo $s . "\n";
}
