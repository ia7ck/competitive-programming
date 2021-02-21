void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n, k;
  rd(n, k);
  auto a = readln.split.to!(int[]);

  auto all = reduce!((res, val) => (res && val == a[0]))(true, a);
  if (all) {
    writeln(0);
    return;
  }
  auto freq = new int[](3 * 10 ^^ 5);
  foreach (val; a)
    freq[val]++;
  auto mn = reduce!(min)(a), mx = reduce!(max)(a);
  int num = 0, s = 0;
  for (int h = mx; h > mn; h--) {
    freq[h] += freq[h + 1];
    if ((s += freq[h]) > k) {
      num++;
      s = freq[h];
    }
  }
  writeln(num + (s > 0));
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
