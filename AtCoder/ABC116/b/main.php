<?php

fscanf(STDIN, '%d', $s);

$seen = [
    $s => true,
];
for ($m = 2; $m <= 1000000; $m++) {
    if ($s % 2 == 0) {
        $s /= 2;
    } else {
        $s = $s * 3 + 1;
    }
    if (isset($seen[$s])) {
        echo $m . "\n";
        exit;
    }
    $seen[$s] = true;
}

exit(1);
