void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int q;
  rd(q);

  while (q--) {
    int n;
    rd(n);
    auto a = readln.split.to!(int[]);
    auto freq = new int[](n);
    foreach (el; a) {
      freq[el - 1] += 1;
    }
    int[] b;
    foreach (f; freq) {
      if (f > 0) {
        b ~= f;
      }
    }
    b.sort!"a>b";
    int last = b[0] + 1;
    long ans = 0;
    foreach (el; b) {
      ans += min(last - 1, el);
      last = min(last - 1, el);
      if (last == 0) {
        break;
      }
    }
    writeln(ans);
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
