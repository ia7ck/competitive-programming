void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  int n; rd(n);
  auto x=readln.split.to!(int[]);
  auto y=readln.split.to!(int[]);

  if(x.reduce!"a+b">=y.reduce!"a+b") writeln("Yes");
  else writeln("No");

}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}