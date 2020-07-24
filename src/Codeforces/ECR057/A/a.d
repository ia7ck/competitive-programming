void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int t;
  rd(t);
  while (t--) {
    int l, r;
    rd(l, r);
    writeln(l, " ", l * 2);
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
