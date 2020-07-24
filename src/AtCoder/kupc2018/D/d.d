void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  bool isEven(int q) {
    writeln("? ", q);
    stdout.flush();
    auto r = readln.chomp.to!(char[]);
    return r[0] == 'e';
  }

  auto ev = isEven(2);
  int lb = 1, ub = 10 ^^ 9 + 1; // lbはX以下の奇数、ubはXより大きい奇数
  while (ub - lb > 2) {
    auto m = (ub + lb) / 2;
    if (m % 2 == 0)
      m++;
    auto __ev = isEven(m);
    if (__ev) {
      (ev ? ub : lb) = m;
    } else {
      (ev ? lb : ub) = m;
    }
  }
  if (ev)
    lb++;
  writeln("! ", lb);
  stdout.flush();
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
