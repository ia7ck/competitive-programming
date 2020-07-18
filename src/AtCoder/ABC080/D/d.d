void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n, C;
  rd(n, C);
  struct T {
    int s, t, c;
  }

  T[] seqs;
  foreach (_; 0 .. n) {
    int s, t, c;
    rd(s, t, c);
    seqs ~= T(s, t, c);
  }
  sort!"a.s<b.s"(seqs);
  struct U {
    int end, c;
  }

  U[] data;
  foreach (seq; seqs) {
    bool found = false;
    foreach (i; 0 .. data.length) {
      if (data[i].end < seq.s) {
        data[i] = U(seq.t, seq.c);
        found = true;
        break;
      } else if (data[i].c == seq.c && data[i].end <= seq.s) {
        data[i] = U(seq.t, seq.c);
        found = true;
        break;
      }
    }
    if (!found) {
      data ~= U(seq.t, seq.c);
    }
  }
  writeln(data.length);

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
