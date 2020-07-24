void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  auto s = readln.chomp;
  auto n = s.length.to!int;

  struct T {
    int l, r;
  }

  T[] a;
  int[int] last_x;
  for (int x = 0, y = 0; x < n; x++) {
    if (s[x] == '\\') {
      last_x[y--] = x;
    } else if (s[x] == '/') {
      if ((++y) in last_x) {
        a ~= T(last_x[y], x);
      }
    }
  }
  if (a.length == 0) {
    writeln(0);
    writeln(0);
    return;
  }
  reverse(a);
  int[] ans;
  int l, r;
  foreach (i; 0 .. a.length) {
    if (i == 0) {
      l = a[i].l;
      r = a[i].r;
      ans ~= (a[i].r - a[i].l);
    } else {
      if (l <= a[i].l && a[i].r <= r) {
        ans[$ - 1] += (a[i].r - a[i].l);
      } else {
        l = a[i].l;
        r = a[i].r;
        ans ~= (a[i].r - a[i].l);
      }
    }
  }
  writeln(ans.reduce!"a+b");
  write(ans.length, " ");
  writefln("%(%s %)", ans.reverse);
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
