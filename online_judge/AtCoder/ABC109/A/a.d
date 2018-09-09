void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  int a, b; rd(a, b);
  for(int c=1; c<=3; c++){
    if((a*b*c)&1){
      writeln("Yes");
      return;
    }
  }

  writeln("No");
}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}
