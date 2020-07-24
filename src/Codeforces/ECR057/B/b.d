void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n;
  rd(n);
  auto s = readln.chomp.to!(char[]);

  auto bf = new int[](26), af = new int[](26);
  int kind = 0;
  foreach (c; s) {
    af[c - 'a']++;
    if (af[c - 'a'] == 1) {
      kind++;
    }
  }
  int ans = 0, mod = 998244353;
  for (int i = 0, j = 0; i < n; i++) { // [i, j) remove
    while (j < n && kind >= 2) {
      if ((--af[s[j++] - 'a']) == 0) {
        kind--;
      }
    }
    if (kind == 1) {
      ans += (n - j + 1);
      ans %= mod;
    }
    if ((++af[s[i] - 'a']) == 1) {
      kind++;
    }
  }
  writeln(ans);
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
