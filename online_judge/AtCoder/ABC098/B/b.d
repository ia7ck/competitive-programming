void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n;
  rd(n);
  auto s = readln.chomp.to!(char[]);

  int mx = 0;
  foreach (i; 1 .. n) {
    bool[char] map;
    foreach (j; 0 .. i) {
      map[s[j]] = true;
    }
    int c = 0;
    foreach (j; i .. n) {
      if (s[j] in map) {
        c++;
        map.remove(s[j]);
      }
    }
    mx = max(mx, c);
  }
  writeln(mx);
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
