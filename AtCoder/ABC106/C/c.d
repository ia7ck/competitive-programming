void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  auto s = readln.chomp.to!(char[]);
  long k;
  rd(k);

  int j = -1;
  for (int i = 0; i < s.length; i++) {
    if (s[i] > '1') {
      j = i;
      break;
    }
  }
  if (j < 0) {
    writeln(1);
  } else {
    if (k <= j) {
      writeln(1);
    } else {
      writeln(s[j]);
    }
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
