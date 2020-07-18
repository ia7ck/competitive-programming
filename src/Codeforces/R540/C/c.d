void main() {
  import std.stdio, std.string, std.conv, std.algorithm;
  import std.exception;

  int n;
  rd(n);
  auto a = readln.split.to!(int[]);
  sort(a);

  if (n == 1) {
    writeln("YES");
    writeln(a[0]);
    return;
  }

  auto freq = new int[](1234);
  foreach (el; a) {
    freq[el]++;
  }
  if (n % 2 == 0) {
    auto mat = new int[][](n, n);
    foreach (i; 0 .. n / 2) {
      foreach (j; 0 .. n / 2) {
        foreach (el; a) {
          if (freq[el] > 0 && freq[el] % 4 == 0) {
            mat[i][j] = mat[n - i - 1][j] = mat[i][n - j - 1] = mat[n - i - 1][n - j - 1] = el;
            freq[el] -= 4;
            break;
          }
        }
      }
    }
    if (freq.any!((el) => (el > 0))) {
      writeln("NO");
    } else {
      writeln("YES");
      writefln("%(%(%s %)\n%)", mat);
    }
  } else {
    auto mat = new int[][](n, n);
    foreach (el; a) {
      if (freq[el] % 4 == 1 || freq[el] % 4 == 3) {
        mat[n / 2][n / 2] = el;
        freq[el]--;
        break;
      }
    }
    if (freq.any!((el) => (el & 1))) {
      writeln("NO");
      return;
    }
    foreach (i; 0 .. n / 2) {
      foreach (j; 0 .. n / 2) {
        foreach (el; a) {
          if (freq[el] > 2) {
            mat[i][j] = mat[n - i - 1][j] = mat[i][n - j - 1] = mat[n - i - 1][n - j - 1] = el;
            freq[el] -= 4;
            break;
          }
        }
      }
    }
    foreach (i; 0 .. n / 2) {
      if (mat[i][n / 2] == 0) {
        foreach (el; a) {
          if (freq[el] > 0 && freq[el] % 2 == 0) {
            mat[i][n / 2] = mat[n - i - 1][n / 2] = el;
            freq[el] -= 2;
            break;
          }
        }
      }
    }
    foreach (j; 0 .. n / 2) {
      if (mat[n / 2][j] == 0) {
        foreach (el; a) {
          if (freq[el] > 0 && freq[el] % 2 == 0) {
            mat[n / 2][j] = mat[n / 2][n - j - 1] = el;
            freq[el] -= 2;
            break;
          }
        }
      }
    }
    if (freq.any!((el) => (el > 0))) {
      writeln("NO");
    } else {
      writeln("YES");
      writefln("%(%(%s %)\n%)", mat);
    }
  }
}

/*

7
5 9 5 4 1 9 8 4 5 1 4 10 7 7 8 4 2 4 4 5 4 4 10 3 4 6 8 1 9 9 5 6 8 7 1 8 6 6 7 5 3 1 1 4 7 2 3 3 8

*/

void rd(T...)(ref T x) {
  import std.stdio : readln;
  import std.string : split;
  import std.conv : to;

  auto l = readln.split;
  assert(l.length == x.length);
  foreach (i, ref e; x)
    e = l[i].to!(typeof(e));
}
