void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  auto s = readln.chomp.to!(char[]);
  auto t = readln.chomp.to!(char[]);

  foreach (i; 0 .. s.length) {
    if (s == t) {
      writeln("Yes");
      return;
    }
    s = s[$ - 1] ~ s[0 .. ($ - 1)];
  }

  writeln("No");

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
