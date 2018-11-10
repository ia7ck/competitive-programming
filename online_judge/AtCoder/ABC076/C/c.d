void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  auto s = readln.chomp.to!(char[]);
  auto t = readln.chomp.to!(char[]);

  const no = "UNRESTORABLE";
  if (t.length > s.length) {
    writeln(no);
    return;
  }
  char[][] cand;
  for (size_t i = 0; i + t.length <= s.length; i++) {
    bool ok = true;
    foreach (j; 0 .. t.length) {
      if (s[i + j] != '?') {
        ok &= s[i + j] == t[j];
      }
    }
    if (ok) {
      auto u = s[0 .. i] ~ t ~ s[i + t.length .. $];
      foreach (ref c; u)
        if (c == '?')
          c = 'a';
      cand ~= u;
    }
  }
  if (cand.length > 0) {
    sort(cand);
    writeln(cand[0]);
  } else {
    writeln(no);
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
