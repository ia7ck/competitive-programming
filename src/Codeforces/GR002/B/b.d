void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n, h;
  rd(n, h);
  auto a = readln.split.to!(int[]);

  a ~= (h + 1);
  int[] b;
  foreach (k, el; a) {
    b ~= el;
    b.sort;
    b.reverse;
    int cur_h = 0;
    bool ok = true;
    foreach (i; 0 .. b.length) {
      if (cur_h + b[i] > h) {
        ok = false;
        break;
      }
      if (i & 1) {
        cur_h += max(b[i - 1], b[i]);
      }
    }
    if (!ok) {
      writeln(k);
      break;
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
