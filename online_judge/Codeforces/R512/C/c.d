void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n;
  rd(n);
  auto str = readln.chomp.to!(char[]);

  auto a = new int[](n);
  foreach (i; 0 .. n)
    a[i] = str[i] - '0';

  int s = a[0];
  foreach (i; 1 .. n) { // sum(a[0, i))
    for (int j = i, t = 0; j < n; j++) {
      while (j < n && t < s) {
        t += a[j++];
      }
      if (s == t) {
        while (j < n && s == t + a[j]) {
          j++;
        }
        if (j == n) {
          writeln("YES");
          return;
        }
      } else {
        break;
      }
      if (j < n) {
        t = a[j];
        while (s == t) {
          j++;
          if (j < n)
            t = a[j];
          else
            t = -1;
        }
        if (j == n) {
          writeln("YES");
          return;
        }
      }
    }
    s += a[i];
  }

  writeln("NO");
}

void rd(T...)(ref T x) {
  import std.stdio, std.string, std.conv;

  auto l = readln.split;
  assert(l.length == x.length);
  foreach (i, ref e; x)
    e = l[i].to!(typeof(e));
}
