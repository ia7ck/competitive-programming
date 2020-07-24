void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  int a, b; rd(a, b);

  if(a+b==15){
    writeln("+");
  }else if(a*b==15){
    writeln("*");
  }else{
    writeln("x");
  }
}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}