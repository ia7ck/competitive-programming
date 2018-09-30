void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n;
  rd(n);
  auto a = readln.split.to!(int[]);

  const int M = 10 ^^ 6;
  auto freq = new int[][](2, M);
  foreach (i, e; a) {
    freq[i & 1][e]++;
  }
  struct P {
    int val, num;
  }

  auto v = new P[][](2);
  foreach (i; 0 .. M) {
    foreach (j; 0 .. 2) {
      if (freq[j][i] > 0) {
        v[j] ~= P(i, freq[j][i]);
      }
    }
  }
  foreach (i; 0 .. 2) {
    v[i] ~= P(0, 0);
    sort!((l, r) => (l.num > r.num))(v[i]);
  }
  int ans;
  if (v[0][0].val != v[1][0].val) {
    ans = (n / 2 - v[0][0].num) + (n / 2 - v[1][0].num);
  } else {
    ans = min((n / 2 - v[0][0].num) + (n / 2 - v[1][1].num),
        (n / 2 - v[0][1].num) + (n / 2 - v[1][0].num));
  }
  writeln(ans);
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
