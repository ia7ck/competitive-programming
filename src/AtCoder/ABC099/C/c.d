void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  int n; rd(n);
  
  int[] a;
  for(int x=6; x<=n; x*=6) a~=x;
  for(int x=9; x<=n; x*=9) a~=x;
  a~=1;

  auto dp=new int[](n+1);
  const int inf=1_000_000_000;
  fill(dp, inf);
  dp[0]=0;
  foreach(i; 0..n)foreach(e; a){
    if(i+e<=n) dp[i+e]=min(dp[i+e], dp[i]+1);
  }
  writeln(dp[n]);
}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}