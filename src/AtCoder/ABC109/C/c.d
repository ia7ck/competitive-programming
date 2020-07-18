void main(){
  import std.stdio, std.string, std.conv, std.algorithm;
  import std.numeric, std.math, std.array;

  int n, x; rd(n, x);
  auto a=readln.split.to!(int[]);

  auto d=a.map!(e=>abs(e-x)).array;
  if(n==1){
    writeln(d[0]);
    return;
  }
  auto g=gcd(d[0], d[1]);
  foreach(i; 2..n) g=gcd(g, d[i]);
  writeln(g);
}


void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}
