void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  int n, k; rd(n, k);
  auto cost=new int[][](k, k);
  foreach(i; 0..k) cost[i]=readln.split.to!(int[]);
  auto a=new int[][](n, n);
  foreach(i; 0..n){
    a[i]=readln.split.to!(int[]);
    a[i].each!((ref e)=>e--);
  }

  auto s=new int[][](k, n*2);
  foreach(c; 0..k)for(int i0=0; i0<=((n-1)*2); i0++){
    for(int j=max(0, i0-n+1), i=min(i0, n-1); i>=0 && j<n; i--, j++){
      s[c][i0]+=cost[a[i][j]][c];
    }
  }
  int mn=1_000_000_000;
  foreach(c1; 0..k)foreach(c2; 0..k)foreach(c3; 0..k){
    if((c1!=c2 && c2!=c3 && c3!=c1)==false) continue;
    int beta=0;
    for(int i=0; i<=(n-1)*2; i++){
      if(i%3==0){
        beta+=s[c1][i];
      }else if(i%3==1){
        beta+=s[c2][i];
      }else{
        beta+=s[c3][i];
      }
    }
    mn=min(mn, beta);
  }

  writeln(mn);
}


void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}