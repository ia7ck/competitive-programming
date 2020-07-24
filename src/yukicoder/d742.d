void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n;
  rd(n);
  auto a = new int[](n);
  foreach (i; 0 .. n)
    rd(a[i]);
  int cnt = 0;
  auto tree = new FenwickTree(n);
  foreach (i; 0 .. n) {
    cnt += i - tree.sum(a[i]);
    tree.add(a[i], 1);
  }
  writeln(cnt);
}

class FenwickTree {
  int n;
  int[] dat;
  this(int n) {
    this.n = n;
    dat.length = n + 1;
  }

  void add(int i, int x) {
    for (int k = i; k <= n; k += k & (-k))
      dat[k] += x;
  }

  int sum(int i) { // [1, i]
    int s = 0;
    for (int k = i; k > 0; k -= k & (-k))
      s += dat[k];
    return s;
  }

  int sum(int i, int j) { // [i, j]
    return sum(j) - sum(i);
  }
}

void rd(T...)(ref T x) {
  import std.stdio : readln;
  import std.string : split;
  import std.conv : to;

  auto l = readln.split;
  assert(l.length == x.length);
  foreach (i, ref e; x)
    e = l[i].to!(typeof(e));
}
