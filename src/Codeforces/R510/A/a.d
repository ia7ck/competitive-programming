void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  int n, m; rd(n); rd(m);
  auto a=new int[](n);
  foreach(i; 0..n) rd(a[i]);

  auto mx=reduce!(max)(a);
  int sub=0;
  foreach(e; a) sub+=(mx-e);
  if(m<=sub){
    writeln(mx, " ", mx+m);
    return;
  }
  auto k2=mx+m;
  m-=sub;
  writeln(mx+(m+n-1)/n, " ", k2);

}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}
