void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  long n, m, k;
  rd(n, m, k);
  if (n * m * 2 % k != 0) {
    writeln("NO");
    return;
  }

  writeln("YES");
  bool odd = true;
  if (k % 2 == 0) {
    k /= 2;
    odd = false;
  }
  import std.numeric : gcd;

  auto g = gcd(n, k), a = n / g;
  k /= g;
  auto b = m / k;
  if (odd) {
    if (a == n) {
      b *= 2;
    } else {
      a *= 2;
    }
  }
  writeln(0, " ", 0);
  writeln(a, " ", 0);
  writeln(0, " ", b);

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
