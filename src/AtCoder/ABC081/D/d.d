void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n;
  rd(n);
  auto a = readln.split.to!(long[]);

  import std.math : abs;

  int arg_abs_max = 0;
  foreach (i; 1 .. n) {
    if (abs(a[arg_abs_max]) < abs(a[i])) {
      arg_abs_max = i;
    }
  }
  struct T {
    int y, x;
    string toString() {
      return format("%d %d", x, y);
    }
  }

  T[] ans;
  foreach (i; 0 .. n) {
    if (i != arg_abs_max) {
      ans ~= T(i + 1, arg_abs_max + 1);
      a[i] += a[arg_abs_max];
    }
  }
  if (a[arg_abs_max] >= 0) {
    foreach (i; 1 .. n) {
      ans ~= T(i + 1, i);
      a[i] += a[i - 1];
    }
  } else {
    foreach_reverse (i; 1 .. n) {
      ans ~= T(i, i + 1);
      a[i - 1] += a[i];
    }
  }
  import std.exception : enforce;

  enforce(ans.length <= n * 2);
  foreach (i; 1 .. n)
    enforce(a[i - 1] <= a[i]);
  writeln(ans.length);
  writefln("%(%s\n%)", ans);
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
