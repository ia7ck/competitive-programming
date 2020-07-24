void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n, g;
  rd(n, g);
  struct P {
    int p, c;
  }

  auto ps = new P[](n);
  foreach (i; 0 .. n)
    rd(ps[i].p, ps[i].c);
  int mn = 1000000000;
  foreach (bit; 0 .. (1 << n)) {
    int s = 0, cnt = 0;
    foreach (i; 0 .. n) {
      if (bit & (1 << i)) {
        s += ps[i].p * (i + 1) * 100 + ps[i].c;
        cnt += ps[i].p;
      }
    }
    if (s >= g) {
      mn = min(mn, cnt);
      continue;
    }
    foreach_reverse (i; 0 .. n) {
      if ((bit & (1 << i)) == 0) {
        if (s + ps[i].p * (i + 1) * 100 >= g) {
          mn = min(mn, cnt + ((g - s) + (i + 1) * 100 - 1) / ((i + 1) * 100));
        }
        break;
      }
    }
  }
  writeln(mn);
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
