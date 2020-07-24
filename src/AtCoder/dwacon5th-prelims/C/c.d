void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n;
  rd(n);
  auto s = readln.chomp.to!(char[]);
  int q;
  rd(q);
  auto ks = readln.split.to!(int[]);
  foreach (k; ks) {
    long d = 0, dm = 0, dmc = 0;
    long m = 0;
    foreach (i; 0 .. k) {
      if (s[i] == 'D') {
        d++;
      } else if (s[i] == 'M') {
        dm += d;
        m++;
      } else if (s[i] == 'C') {
        dmc += dm;
      }
    }
    foreach (i; k .. n) {
      if (s[i - k] == 'D') {
        d--;
        dm -= m;
      } else if (s[i - k] == 'M') {
        m--;
      }
      if (s[i] == 'D') {
        d++;
      } else if (s[i] == 'M') {
        dm += d;
        m++;
      } else if (s[i] == 'C') {
        dmc += dm;
      }
    }
    writeln(dmc);
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
