void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n;
  rd(n);
  auto a = readln.split.to!(int[]);

  auto st = new SegmentTree(n);
  auto ai = new int[][](n);
  foreach (i; 0 .. n) {
    ai[i] = [a[i], i];
  }
  ai.sort!((l, r) => (l[0] == r[0] ? l[1] > r[1] : l[0] < r[0]));
  foreach (e; ai) {
    auto i = e[1], aa = e[0];
    auto u = st.get_max(0, i);
    st.update(i, u.val + 1, u.sum + aa);
  }
  auto ans = st.get_max(0, n);
  writeln(ans.sum);
}

class SegmentTree {
  struct Node {
    int val;
    long sum;
  }

  Node max(const Node l, const Node r) {
    if (l.val < r.val) {
      return r;
    } else if (l.val > r.val) {
      return l;
    } else {
      if (l.sum <= r.sum) {
        return r;
      } else {
        return l;
      }
    }
  }

  int n = 1;
  const auto e = Node(0, 0);
  Node[] dat;
  this(int N) {
    while (n < N)
      n *= 2;
    dat.length = n * 2 - 1;
  }

  void update(int i, int val, long sum) {
    i += n - 1;
    dat[i] = Node(val, sum);
    while (i > 0) {
      i = (i - 1) / 2;
      dat[i] = max(dat[i * 2 + 1], dat[i * 2 + 2]);
    }
  }

  Node get_max(int ql, int qr) {
    return get_max(ql, qr, 0, 0, n);
  }

  Node get_max(int ql, int qr, int i, int il, int ir) {
    if (qr <= il || ir <= ql)
      return e;
    if (ql <= il && ir <= qr)
      return dat[i];
    int m = (il + ir) / 2;
    return max(get_max(ql, qr, i * 2 + 1, il, m), get_max(ql, qr, i * 2 + 2, m, ir));
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
