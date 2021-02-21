void main(){
  import std.stdio, std.string, std.conv, std.algorithm;
  import std.array;

  int n; rd(n);
  auto g=readln.split.to!(int[]).map!((e)=>(e-1)).array;

  int f(int i, bool[] vis){
    if(vis[i]) return i;
    vis[i]=true;
    return f(g[i], vis);
  }

  foreach(i; 0..n){
    auto vis=new bool[](n);
    writeln(f(i, vis)+1);
  }
}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}
