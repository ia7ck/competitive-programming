void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  long n, m; rd(n, m);

  if(n>m) swap(n, m);
  if((m-n)>=2){
    writeln(0);
    return;
  }
  long tot=1, mod=10^^9+7;
  for(int i=1; i<=n; i++) (tot*=i)%=mod;
  for(int i=1; i<=m; i++) (tot*=i)%=mod;
  if(n==m) (tot*=2)%=mod;
  writeln(tot);
}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}
