void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int t;
  rd(t);
  while (t--) {
    auto s = readln.chomp.to!(char[]);
    int d = 0, l = 0, u = 0;
    foreach (c; s) {
      if ('0' <= c && c <= '9') {
        d++;
      }
      if ('a' <= c && c <= 'z')
        l++;
      if ('A' <= c && c <= 'Z')
        u++;
    }
    if (d > 0 && l > 0 && u > 0) {

      writeln(s);

      continue;

    }
    if (d == 0) {
      foreach (i; 0 .. s.length) {
        auto c = s[i];
        if ('a' <= c && c <= 'z' && l > 1) {
          s[i] = '0';
          break;
        }
        if ('A' <= c && c <= 'Z' && u > 1) {
          s[i] = '0';
          break;
        }
      }
    }
    if (l == 0) {
      foreach (i; 0 .. s.length) {
        auto c = s[i];
        if ('0' <= c && c <= '9' && d > 1) {
          s[i] = 'a';
          break;
        }
        if ('A' <= c && c <= 'Z' && u > 1) {
          s[i] = 'a';
          break;
        }
      }
    }
    if (u == 0) {
      foreach (i; 0 .. s.length) {
        auto c = s[i];
        if ('0' <= c && c <= '9' && d > 1) {
          s[i] = 'A';
          break;
        }
        if ('a' <= c && c <= 'z' && l > 1) {
          s[i] = 'A';
          break;
        }
      }
    }
    writeln(s);
  }

}

void rd(T...)(ref T x) {
  import std.stdio, std.string, std.conv;

  auto l = readln.split;
  assert(l.length == x.length);
  foreach (i, ref e; x)
    e = l[i].to!(typeof(e));
}
