void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  int n, m; rd(n, m);
  auto a=readln.split.to!(int[]);
  auto b=readln.split.to!(int[]);

  auto ax=a.reduce!((r, e)=>(r^e)),
       bx=b.reduce!((r, e)=>(r^e));
  if(ax!=bx){writeln("NO"); return;}
  writeln("YES");
  write(a[0]^b[0]^bx, " ");
  writefln("%(%s %)", b[1..$]);
  foreach(i; 1..n){
    write(a[i], " ");
    auto zero=new int[](m-1);
    writefln("%(%s %)", zero);
  }
}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}