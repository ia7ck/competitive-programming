void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  int n; rd(n);
  auto a=readln.split.to!(long[]);

  auto dp=new int[](n);
  dp[0]=1;
  foreach(i; 1..n){
    dp[i]=1;
    if(a[i]<=a[i-1]*2){
      dp[i]=max(dp[i], dp[i-1]+1);
    }
  }
  writeln(dp.reduce!(max));
}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}
