void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  int n; long x; rd(n, x);
  auto a=readln.split.to!(long[]);

  sort(a);
  int num=0;
  foreach(e; a){
    if(x>=e) num++;
    x-=e;
  }
  if(x>0) num--;
  writeln(num);
}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}
