void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n;
  rd(n);
  auto a = readln.split.to!(int[]);

  auto freq = new int[](105);
  foreach (e; a)
    freq[e]++;
  int nice = 0;
  foreach (e; freq) {
    if (e == 1)
      nice++;
  }

  bool check(char[] s) {
    auto fa = new int[](101), fb = new int[](101);
    foreach (i, c; s) {
      if (c == 'A')
        fa[a[i]]++;
      else
        fb[a[i]]++;
    }

    auto na = reduce!((r, e) => (r + (e == 1 ? 1 : 0)))(0, fa);
    auto nb = reduce!((r, e) => (r + (e == 1 ? 1 : 0)))(0, fb);
    return na == nb;
  }

  if (nice & 1) {
    if (n == nice) {
      writeln("NO");
      return;
    }
    bool o = reduce!((r, e) => (r || (e >= 3)))(false, freq);
    if (o == false) {
      writeln("NO");
      return;
    }
    writeln("YES");
    int _nice = 0;
    char[] ans;
    bool done = false;
    foreach (i; 0 .. n) {
      if (freq[a[i]] >= 3) {
        if (done == false) {
          ans ~= 'A';
          done = true;
        } else {
          ans ~= 'B';
        }
      } else if (_nice * 2 < nice - 1) {
        if (freq[a[i]] == 1) {
          ans ~= 'A';
          _nice++;
        } else {
          ans ~= 'B';
        }
      } else {
        ans ~= 'B';
      }
    }
    assert(check(ans));
    writeln(ans);
    return;
  }
  writeln("YES");
  int _nice = 0;
  char[] ans;
  foreach (i; 0 .. n) {
    if (_nice * 2 < nice) {
      if (freq[a[i]] == 1) {
        ans ~= 'A';
        _nice++;
      } else {
        ans ~= 'B';
      }
    } else {
      ans ~= 'B';
    }
  }
  assert(check(ans));
  writeln(ans);
}

void rd(T...)(ref T x) {
  import std.stdio, std.string, std.conv;

  auto l = readln.split;
  assert(l.length == x.length);
  foreach (i, ref e; x)
    e = l[i].to!(typeof(e));
}
