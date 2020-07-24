#include <random>

std::mt19937 mt(0);
std::uniform_int_distribution<int> dist(1, 0x7fffffff);

struct Node {
  int val;
  int size; // 部分木のサイズ
  struct Node *left, *right;
  Node(int val) : val(val), size(1) { left = right = nullptr; };
};

int size(Node *node) { return node == nullptr ? 0 : node->size; }
Node *update(Node *node) {
  node->size = size(node->left) + size(node->right) + 1;
  return node;
}
Node *merge(Node *left, Node *right) {
  if (left == nullptr) return right;
  if (right == nullptr) return left;
  if (dist(mt) % (size(left) + size(right)) < size(left)) {
    left->right = merge(left->right, right);
    return update(left);
  } else {
    right->left = merge(left, right->left);
    return update(right);
  }
}
// 左にk個残す
std::pair<Node *, Node *> split(Node *node, int k) {
  if (node == nullptr) return {node, node};
  if (size(node->left) >= k) {
    auto node_pair = split(node->left, k);
    node->left = node_pair.second; // 左k個に入れなかった頂点を取り込む
    return {node_pair.first, update(node)};
  } else {
    auto node_pair = split(node->right, k - (size(node->left) + 1));
    node->right = node_pair.first;
    return {update(node), node_pair.second};
  }
}
// lowerbound
int count_less(Node *node, int x) {
  if (node == nullptr) return 0; // ??
  if (node->val < x) {
    // 左はすべてx未満確定
    return size(node->left) + count_less(node->right, x) + 1;
  } else {
    return count_less(node->left, x);
  }
}
Node *insert(Node *node, int x) {
  auto node_pair = split(node, count_less(node, x));
  return merge(node_pair.first, merge(new Node(x), node_pair.second));
}
// 1-indexed
Node *get_kth(Node *node, int k) {
  if (node == nullptr) return node; // ??
  if (size(node) < k) return nullptr;
  auto l_size = size(node->left);
  if (l_size + 1 == k) return node;
  if (l_size + 1 < k) return get_kth(node->right, k - (l_size + 1));
  return get_kth(node->left, k);
}
Node *erase(Node *node, int x) {
  auto node_pair = split(node, count_less(node, x));
  return merge(node_pair.first, split(node_pair.second, 1).second);
}

#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)n; i++)

int main() {

  Node *node = nullptr;
  int q;
  cin >> q;
  while (q--) {
    int t, x;
    cin >> t >> x;
    if (t == 1) {
      node = insert(node, x);
    } else {
      auto n = get_kth(node, x);
      cout << n->val << endl;
      node = erase(node, n->val);
    }
  }

  return 0;
}
