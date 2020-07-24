void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  int k; rd(k);

  struct E{int u, v, w;}
  E[] edges;
  int n=2;
  void f(int x){
    if(x==2){
      edges~=E(1, 2, 0);
      edges~=E(1, 2, 1);
    }else{
      if(x&1){
        f(x-1);
        edges~=E(1, n, x-1);
      }else{
        f(x/2);
        foreach(ref e; edges) e.w*=2;
        edges~=E(n, n+1, 0);
        edges~=E(n, n+1, 1);
        n++;
      }
    }
  }
  f(k);
  writeln(n, " ", edges.length);
  foreach(e; edges){
    writeln(e.u, " ", e.v, " ", e.w);
  }
}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}
