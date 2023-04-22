<?php

fscanf(STDIN, '%s', $S);

foreach (range('a', 'z') as $c) {
    if (strpos($S, $c) === false) {
        echo $c . "\n";
        exit;
    }
}

echo "None\n";
