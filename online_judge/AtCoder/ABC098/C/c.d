void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n;
  rd(n);
  auto s = readln.chomp.to!(char[]);

  int sum = 0;
  foreach (c; s[1 .. $]) {
    if (c == 'E')
      sum++;
  }
  auto mn = sum;
  foreach (i; 1 .. n) {
    if (s[i - 1] == 'W')
      sum++;
    if (s[i] == 'E')
      sum--;
    mn = min(mn, sum);
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
