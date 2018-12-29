void main() {
  import std.stdio, std.string, std.conv, std.algorithm;
  import std.exception;

  int t;
  rd(t);
  while (t--) {
    int a;
    rd(a);
    bool ok = false;
    for (int n = 3; n <= 360; n++) {
      if (360 % n == 0) {
        auto x = 180 - 360 / n;
        if (a * (n - 2) % x == 0) {
          if (a * (n - 2) / x <= (n - 2)) {
            writeln(n);
            ok = true;
            break;
          }
        }
      }
    }
    enforce(ok);
  }

}

/*

3 60 60
4 90 45
5 108 36
6 120 30
7 180-360/7 x/5
8 135 135/6
.
.
.
n 180-360/n:=x x/(n-2)

*/

void rd(T...)(ref T x) {
  import std.stdio : readln;
  import std.string : split;
  import std.conv : to;

  auto l = readln.split;
  assert(l.length == x.length);
  foreach (i, ref e; x)
    e = l[i].to!(typeof(e));
}
