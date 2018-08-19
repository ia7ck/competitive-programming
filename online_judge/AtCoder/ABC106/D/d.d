import std.stdio, std.string, std.conv, std.algorithm;

void main(){

  int n, m, q; rd(n, m, q);
  auto cul=new int[][](n+1, n+1);
  foreach(_; 0..m){
    int l, r; rd(l, r);
    cul[l][r]++;
  }
  for(int i=1; i<=n; i++){
    foreach(j; 0..n) cul[i][j+1]+=cul[i][j];
  }
  while(q--){
    int l, r; rd(l, r);
    int num=0;
    for(int i=l; i<=r; i++){
      num+=cul[i][r]-cul[i][i-1];
    }
    writeln(num);
  }

}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}
