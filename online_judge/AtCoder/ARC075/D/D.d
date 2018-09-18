void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  int n; long a, b; rd(n, a, b);
  auto h=new long[](n);
  foreach(i; 0..n) rd(h[i]);

  bool c(long k){
    long num=0;
    // foreach(e; h) num+=max(0, (e-b*k)/a);
    return num<=k;
  }

  long ng=0, ok=10000000000;
  while(ok-ng>1){
    auto m=(ng+ok)/2;
    (c(m) ? ok : ng = m);
  }

  writeln(ok);
}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}
