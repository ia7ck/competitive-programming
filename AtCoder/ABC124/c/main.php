<?php

fscanf(STDIN, '%s', $S);

$ans = 0;
for ($i = 1; $i < strlen($S); $i++) {
    if ($S[$i - 1] == $S[$i]) {
        $ans += 1;
        $S[$i] = $S[$i] == '1' ? '0' : '1';
    }
}

echo $ans . "\n";
