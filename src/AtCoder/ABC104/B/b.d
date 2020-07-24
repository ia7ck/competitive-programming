void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  auto s = readln.chomp.to!(char[]);
  if (s[0] != 'A') {
    writeln("WA");
    return;
  }
  if (s.count('A') != 1 || s.count('C') != 1) {
    writeln("WA");
    return;
  }
  foreach (c; s) {
    if (c != 'A' && c != 'C') {
      if ('A' <= c && c <= 'Z') {
        writeln("WA");
        return;
      }
    }
  }
  if (s[2 .. ($ - 1)].count('C') != 1) {
    writeln("WA");
    return;
  }

  writeln("AC");

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
