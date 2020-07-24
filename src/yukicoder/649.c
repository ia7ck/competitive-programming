#include <stdio.h>
#include <stdlib.h>

unsigned int rand_int(void) {
  static unsigned int x = 123456789, y = 362436069, z = 521288629, w = 88675123;
  unsigned int t;
  t = (x ^ (x << 11));
  x = y;
  y = z;
  z = w;
  return (w = (w ^ (w >> 19)) ^ (t ^ (t >> 8)));
}

typedef long long i64;

typedef struct Node {
  i64 val;
  int size;
  struct Node *left;
  struct Node *right;
} Node;
Node *create_node(i64 val) {
  Node *ret = (Node *)malloc(sizeof(Node));
  ret->val = val;
  ret->size = 1;
  ret->left = ret->right = NULL;
  return ret;
}

typedef struct Pair {
  Node *left, *right;
} Pair;
Pair make_pair(Node *left, Node *right) {
  Pair pair = {left, right};
  return pair;
}

int size(Node *node) { return node == NULL ? 0 : node->size; }

Node *update(Node *node) {
  node->size = 1 + size(node->left) + size(node->right);
  return node;
}

Node *merge(Node *left, Node *right) {
  if (left == NULL) {
    return right;
  } else if (right == NULL) {
    return left;
  }
  if (rand_int() % (size(left) + size(right)) < (unsigned int)size(left)) {
    left->right = merge(left->right, right);
    return update(left);
  } else {
    right->left = merge(left, right->left);
    return update(right);
  }
}

Pair split(Node *root, int k) {
  if (root == NULL) return make_pair(root, root);
  if (k <= size(root->left)) {
    Pair nodes = split(root->left, k);
    root->left = nodes.right;
    return make_pair(nodes.left, update(root));
  } else {
    Pair nodes = split(root->right, k - size(root->left) - 1);
    root->right = nodes.left;
    return make_pair(update(root), nodes.right);
  }
}

int count_less(Node *root, i64 x) {
  if (root == NULL) return 0;
  if (root->val < x) {
    return count_less(root->right, x) + size(root->left) + 1;
  } else {
    return count_less(root->left, x);
  }
}

Node *insert(Node *root, i64 x) {
  Pair nodes = split(root, count_less(root, x));
  return merge(merge(nodes.left, create_node(x)), nodes.right);
}

Node *erase(Node *root, i64 x) {
  Pair nodes = split(root, count_less(root, x));
  if (nodes.right != NULL) {
    return merge(nodes.left, split(nodes.right, 1).right);
  } else {
    return merge(nodes.left, nodes.right);
  }
}

Node *get_kth(Node *root, int k) {
  if (root == NULL || size(root) < k) return NULL;
  int left_size = size(root->left);
  if (left_size + 1 == k) {
    return root;
  } else if (left_size >= k) {
    return get_kth(root->left, k);
  } else {
    return get_kth(root->right, k - left_size - 1);
  }
}

int main(void) {

  Node *root = NULL;
  int q, k;
  scanf("%d %d", &q, &k);
  while (q--) {
    int t;
    scanf("%d", &t);
    if (t == 1) {
      i64 v;
      scanf("%lld", &v);
      root = insert(root, v);
    } else {
      Node *kth = get_kth(root, k);
      if (kth == NULL) {
        printf("-1\n");
      } else {
        printf("%lld\n", kth->val);
        root = erase(root, kth->val);
      }
    }
  }
  return 0;
}
