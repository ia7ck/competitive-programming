<?php

function nextPermutation(array &$a): bool
{
    $n = count($a);

    $i = $n - 1;
    while ($i >= 1 && $a[$i - 1] >= $a[$i]) {
        $i -= 1;
    }
    if ($i == 0) {
        return false;
    }

    // $a[$i - 1] < $a[$i]

    $j = $n - 1;
    while ($a[$i - 1] >= $a[$j]) {
        $j -= 1;
    }

    // $i - 1 < $j

    // swap
    [$a[$i - 1], $a[$j]] = [$a[$j], $a[$i - 1]];

    // reverse
    for ($k = $i; $k < $n; $k++) {
        $r = $n - ($k - $i) - 1;
        if ($k >= $r) {
            break;
        }
        [$a[$k], $a[$r]] = [$a[$r], $a[$k]];
    }

    return true;
}

fscanf(STDIN, '%d', $N);

$points = [];
for ($i = 0; $i < $N; $i++) {
    fscanf(STDIN, '%d %d', $x, $y);
    $points[] = compact('x', 'y');
}
$sum = 0.0;
$m = 0;
$ord = range(0, $N - 1);
while (true) {
    $d = 0.0;
    for ($i = 0; $i + 1 < $N; $i++) {
        $p = $points[$ord[$i]];
        $q = $points[$ord[$i + 1]];
        $d += hypot($p['x'] - $q['x'], $p['y'] - $q['y']);
    }
    $sum += $d;
    $m += 1;
    if (nextPermutation($ord) == false) {
        break;
    }
}
$ans = $sum / $m;
printf("%.10f\n", $ans);
