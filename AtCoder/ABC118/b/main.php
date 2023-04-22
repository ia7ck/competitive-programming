<?php

fscanf(STDIN, '%d %d', $N, $M);

$count = array_fill(1, $M, 0);
for ($i = 0; $i < $N; $i++) {
    $a = array_map('intval', explode(' ', fgets(STDIN)));
    for ($j = 1; $j < count($a); $j++) {
        $count[$a[$j]] += 1;
    }
}
$ans = count(array_filter($count, function ($c) use ($N) {
    return $c == $N;
}));
echo $ans . "\n";
