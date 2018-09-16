void main(){
  import std.stdio, std.string, std.conv, std.algorithm;
  import std.numeric;

  long a, b, x, y; rd(a, b, x, y);

  auto g=gcd(x, y);
  x/=g; y/=g;
  writeln(min(a/x, b/y));
}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}
