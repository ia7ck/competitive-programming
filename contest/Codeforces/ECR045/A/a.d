void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  long n, m, a, b; rd(n, m, a, b);
  
  long x=(((n+m-1)/m)*m-n)*a;
  long y=(n-(n/m)*m)*b;

  writeln(min(x, y));
}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}