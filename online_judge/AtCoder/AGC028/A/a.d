void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  long n, m;
  rd(n, m);
  auto s = readln.chomp.to!(char[]);
  auto t = readln.chomp.to!(char[]);

  import std.numeric : gcd;

  auto l = n * m / gcd(n, m);
  char[long] ch;
  foreach (i; 0 .. n) {
    auto j = i * (l / n);
    ch[j] = s[i];
  }
  foreach (i; 0 .. m) {
    auto j = i * (l / m);
    if (j in ch) {
      if (ch[j] != t[i]) {
        writeln(-1);
        return;
      }
    }
  }
  writeln(l);
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
