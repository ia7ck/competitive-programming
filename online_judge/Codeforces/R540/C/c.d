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
    int k = 0;
    foreach (i; 0 .. n / 2) {
      foreach (j; 0 .. n / 2) {
        if (k < n * n) {
          if (freq[a[k]] > 0 && freq[a[k]] % 4 == 0) {
            mat[i][j] = mat[n - i - 1][j] = mat[i][n - j - 1] = mat[n - i - 1][n - j - 1] = a[k];
            freq[a[k]] -= 4;
            k += 4;
          }
        }
      }
    }
    if (k != n * n) {
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
    foreach (i; 0 .. n / 2) {
      foreach (el; a) {
        if (freq[el] % 4 == 2) {
          mat[i][n / 2] = mat[n - i - 1][n / 2] = el;
          freq[el] -= 2;
          break;
        }
      }
    }
    foreach (j; 0 .. n / 2) {
      foreach (el; a) {
        if (freq[el] % 4 == 2) {
          mat[n / 2][j] = mat[n / 2][n - j - 1] = el;
          freq[el] -= 2;
          break;
        }
      }
    }
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
    if (freq.any!((el) => (el != 0))) {
      writeln("NO");
    } else {
      writeln("YES");
      foreach (i; 0 .. n) {
        foreach (j; 0 .. n) {
          enforce(mat[i][j] > 0);
          enforce(mat[i][j] == mat[n - i - 1][j]);
          enforce(mat[i][j] == mat[i][n - j - 1]);
          enforce(mat[i][j] == mat[n - i - 1][n - j - 1]);
        }
      }
      writefln("%(%(%s %)\n%)", mat);
    }
  }
}

/*

7
1 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 3 3

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
