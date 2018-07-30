void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  int n, k; rd(n, k);
  auto a=new long[](n);
  foreach(i; 0..n) rd(a[i]);

  long res=reduce!"a+b"(a);
  writeln(res);
  for(auto l=0, r=0, s=0L; l<n && r<n; ){ // sum[l, r)<=0
    writeln("st: ", l, " ", r);
    while(r<n && s+a[r]<=0) s+=a[r++];
    writeln("ed: ", l, " ", r);
    if(r-l>=k) res-=s, s=0, l=r;
    else l=++r;
  }
  writeln(res);
}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}