void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  long a, b; rd(a, b);

  import std.math;
  auto c=max(a, b);
  writefln("%.18f", PI*4/3.0*c*c*c);

}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}
