void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  auto s = readln.chomp.to!(char[]);

  int need = 1;
  char cur = s[0];
  foreach (c; s[1 .. $]) {
    if (c <= cur) {
      cur = c;
      need++;
    }
  }
  writeln(need);
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
