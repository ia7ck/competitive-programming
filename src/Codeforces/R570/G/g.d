void main() {
  import std.stdio, std.string, std.conv, std.algorithm;
  import std.container.rbtree;

  int q;
  rd(q);

  struct P {
    int f, c;
  }

  while (q--) {
    int n;
    rd(n);
    auto freq = new int[](n), cnt1s = new int[](n);
    foreach (i; 0 .. n) {
      int a, t;
      rd(a, t);
      freq[a - 1] += 1;
      if (t == 1) {
        cnt1s[a - 1] += 1;
      }
    }
    auto list = new int[][](n + 1);
    foreach (i; 0 .. n) {
      if (freq[i] > 0) {
        list[freq[i]] ~= cnt1s[i];
      }
    }
    int[] b;
    foreach (i; 0 .. n) {
      if (freq[i] > 0) {
        b ~= freq[i];
      }
    }
    b.sort!"a > b";
    int last = b[0] + 1;
    long ans = 0, sum1s = 0;
    auto rbt = new RedBlackTree!(int, "a>b", true);
    foreach (el; b) {
      auto cur = min(last - 1, el);
      ans += cur;
      foreach (cnt; list[cur]) {
        rbt.insert(cnt);
      }
      auto mx1 = rbt.front;
      sum1s += min(mx1, cur);
      rbt.removeKey(mx1);
      last = cur;
      if (last == 0) {
        break;
      }
    }
    writeln(ans, " ", sum1s);
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
