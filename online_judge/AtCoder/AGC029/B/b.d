void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n;
  rd(n);
  auto a = readln.split.to!(long[]);

  sort(a);
  int[long] freq;
  foreach (x; a) {
    if (x in freq) {
      freq[x]++;
    } else {
      freq[x] = 1;
    }
  }
  int ans = 0;
  foreach_reverse (x; a) {
    if (x in freq) {
      if ((--freq[x]) == 0) {
        freq.remove(x);
      }
      long p2 = 1;
      long[] cand;
      while (p2 <= (1L << 31)) {
        if (p2 - x >= 1) {
          cand ~= (p2 - x);
        }
        p2 *= 2;
      }
      foreach_reverse (y; cand) {
        if (y in freq) {
          ans++;
          if ((--freq[y]) == 0) {
            freq.remove(y);
          }
          break;
        }
      }
    }
  }
  writeln(ans);
}

void rd(T...)(ref T x) {
  import std.stdio, std.string, std.conv;

  auto l = readln.split;
  assert(l.length == x.length);
  foreach (i, ref e; x)
    e = l[i].to!(typeof(e));
}
