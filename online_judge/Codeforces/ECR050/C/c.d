void main() {
  import std.stdio, std.string, std.conv, std.algorithm;
  import std.array;

  int t;
  rd(t);
  while (t--) {
    auto args = readln.split.to!(long[]);
    long solve(long s) {
      int[] n;
      do {
        n = s % 10 ~ n;
        s /= 10;
      }
      while (s);
      auto memo = new long[][][](n.length + 1, 2, n.length + 1);
      foreach (i; 0 .. (n.length + 1))
        foreach (j; 0 .. 2)
          foreach (k; 0 .. (n.length + 1))
            memo[i][j][k] = -1;
      long f(size_t i, bool less, int count) {
        if (i == n.length) {
          return count <= 3;
        } else {
          if (memo[i][less][count] >= 0)
            return memo[i][less][count];
          long ret = 0;
          auto digit = less ? 9 : n[i];
          for (int d = 0; d <= digit; d++) {
            auto l = less || (d < digit);
            auto c = count + (d > 0);
            ret += f(i + 1, l, c);
          }
          return memo[i][less][count] = ret;
        }
      }

      return f(0, 0, 0);
    }

    writeln(solve(args[1]) - solve(args[0] - 1));
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
