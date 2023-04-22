<?php

class Node
{
    public ?Node $left = null;
    public ?Node $right = null;
    public int $key;
    public int $priority;
    public function __construct(int $key)
    {
        $this->key = $key;
        $this->priority = mt_rand();
    }
}

class Treap
{
    private ?Node $root = null;

    private function merge(?Node $left, ?Node $right): ?Node
    {
        if ($left === null) {
            return $right;
        }
        if ($right === null) {
            return $left;
        }
        if ($left->priority > $right->priority) {
            $left->right = $this->merge($left->right, $right);
            return $left;
        } else {
            $right->left = $this->merge($left, $right->left);
            return $right;
        }
    }

    private function split(?Node $node, int $key): array
    {
        if ($node === null) {
            return [null, null];
        }
        if ($node->key < $key) {
            [$node->right, $right] = $this->split($node->right, $key);
            return [$node, $right];
        } else {
            [$left, $node->left] = $this->split($node->left, $key);
            return [$left, $node];
        }
    }

    public function insert(int $key): bool
    {
        [$left, $right] = $this->split($this->root, $key);
        if ($right !== null && $right->key == $key) {
            return false;
        }
        $this->root = $this->merge($this->merge($left, new Node($key)), $right);
        return true;
    }

    // public function erase(int $key)
    // {
    //     [$left, $_root] = $this->split($this->root, $key);
    //     [$_root, $right] = $this->split($_root, $key + 1);
    //     if ($_root !== null && $_root->key != $key) {
    //         return false;
    //     }
    //     $this->root = $this->merge($left, $right);
    //     return true;
    // }

    public function greaterThan(int $key): ?int
    {
        return $this->_greaterThan($this->root, $key);
    }

    private function _greaterThan(?Node $node, int $key): ?int
    {
        if ($node === null) {
            return null;
        }
        if ($node->key <= $key) {
            return $this->_greaterThan($node->right, $key);
        } else {
            return $this->_greaterThan($node->left, $key) ?? $node->key;
        }
    }

    public function lessThan(int $key): ?int
    {
        return $this->_lessThan($this->root, $key);
    }

    private function _lessThan(?Node $node, int $key): ?int
    {
        if ($node === null) {
            return null;
        }
        if ($node->key >= $key) {
            return $this->_lessThan($node->left, $key);
        } else {
            return $this->_lessThan($node->right, $key) ?? $node->key;
        }
    }
}

fscanf(STDIN, '%d %d', $L, $Q);
$queries = [];
for ($i = 0; $i < $Q; $i++) {
    fscanf(STDIN, '%d %d', $c, $x);
    $queries[] = compact('c', 'x');
}

$treap = new Treap();
foreach ($queries as ['c' => $c, 'x' => $x]) {
    if ($c == 1) {
        $success = $treap->insert($x);
        $success || exit(1);
    } else {
        $l = $treap->lessThan($x) ?? 0;
        $r = $treap->greaterThan($x) ?? $L;
        $ans = $r - $l;
        echo $ans . "\n";
    }
}
