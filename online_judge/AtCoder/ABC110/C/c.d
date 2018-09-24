void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  auto s = readln.chomp.to!(char[]);
  auto t = readln.chomp.to!(char[]);

  char[char] map;
  foreach (i; 0 .. s.length) {
    if (s[i] in map) {
      if (t[i] != map[s[i]]) {
        writeln("No");
        return;
      }
    } else {
      map[s[i]] = t[i];
    }
  }
  char[char] map2;
  foreach (i; 0 .. t.length) {
    if (t[i] in map2) {
      if (s[i] != map2[t[i]]) {
        writeln("No");
        return;
      }
    } else {
      map2[t[i]] = s[i];
    }
  }
  writeln("Yes");
}

void rd(T...)(ref T x) {
  import std.stdio, std.string, std.conv;

  auto l = readln.split;
  assert(l.length == x.length);
  foreach (i, ref e; x)
    e = l[i].to!(typeof(e));
}
