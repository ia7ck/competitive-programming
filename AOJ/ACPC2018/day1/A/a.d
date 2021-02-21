void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  int n, m; rd(n, m);
  auto a=readln.split.to!(int[]);

  auto b=new int[](n);
  foreach(e; a) b[e-1]=1;
  int c=0;
  bool f(){
    return reduce!((r, e)=>(r && e==1))(true, b[0..m]);
  }
  foreach_reverse(i; 0..n){
    if(f()) break;
    if(b[i]){
      foreach(j; 0..n){
        if(b[j]==0){
          swap(b[j], b[i]);
          break;
        }
      }
      c++;
    }
  }
  writeln(c);
}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}
