void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  import std.exception : enforce;
  import std.math : abs;

  int n;
  rd(n);
  auto xs=new int[](n);
  auto ys=new int[](n);
  int odd, even;
  foreach(i; 0..n){
    rd(xs[i], ys[i]);
    if((abs(xs[i])+abs(ys[i]))&1) odd++;
    else even++;
  }
  if(odd*even>0){
    writeln(-1);
    return;
  }
  int m=40;
  int[] d;
  if(odd>0){
    foreach(i; 0..m){
      if(i==0) d~=2;
      else d~=1;
    }
  }else{
    foreach(i; 0..m){
      d~=1;
    }
  }
  writeln(m);
  writefln("%(%s %)", d);
  foreach(i; 0..n) {
    auto x=xs[i], y=ys[i];
    enforce(-10 <= x && x <= 10);
    enforce(-10 <= y && y <= 10);
    char[] moves;
    if ((abs(x) + abs(y)) & 1) {
      moves ~= 'R';
      moves ~= 'L';
      // (0, 0) -> (1, 0)
      foreach (_; 0 .. abs(x - 1)) {
        moves ~= (x < 1 ? 'L' : 'R');
      }
    } else {
      foreach (_; 0 .. abs(x)) {
        moves ~= (x < 0 ? 'L' : 'R');
      }
    }
    foreach (_; 0 .. abs(y)) {
      moves ~= (y < 0 ? 'D' : 'U');
    }
    int r = m - moves.length.to!(int);
    assert(r >= 0 && r % 2 == 0);
    foreach (_; 0 .. (r / 2)) {
      moves ~= 'L';
      moves ~= 'R';
    }
    writeln(moves);
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
