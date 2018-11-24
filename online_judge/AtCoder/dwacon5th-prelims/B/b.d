void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n, k;
  rd(n, k);
  auto a = readln.split.to!(long[]);

  long[] bits;
  foreach (i; 0 .. n) {
    long s = 0;
    foreach (j; i .. n) {
      s += a[j];
      bits ~= s;
    }
  }
  auto good = new bool[](bits.length);
  fill(good, true);
  foreach_reverse (i; 0 .. 42) {
    int m = 0;
    foreach (j, bit; bits) {
      if (good[j]) {
        if (bit & (1L << i)) {
          m++;
        }
      }
    }
    if (m >= k) {
      foreach (j, bit; bits) {
        if (good[j]) {
          if (!(bit & (1L << i))) {
            good[j] = false;
          }
        }
      }
      if (m == k) {
        long ans = (1L << 42) - 1;
        foreach (j, bit; bits) {
          if (good[j]) {
            ans &= bit;
          }
        }
        writeln(ans);
        return;
      }
    }
  }
  long ans = (1L << 42) - 1;
  foreach (j, bit; bits) {
    if (good[j]) {
      ans &= bit;
      if (--k == 0) {
        writeln(ans);
        return;
      }
    }
  }
  import std.exception : enforce;

  enforce(false);
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
