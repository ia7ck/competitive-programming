void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  int n; rd(n);

  if(n==1){
    writeln(0);
    return;
  }
  if(n==2){
    writeln(1);
    return;
  }

  static pp=mp();
  int cnt=0;
  for(int p=1; p<=n; p++){
    if(pp[p] && pp[p+2]){
      cnt++;
    }
  }
  writeln(cnt*2);
}

bool[] mp(){
  const int m=10000000+10;
  auto ret=new bool[](m);
  foreach(i; 0..m) ret[i]=true;
  ret[0]=ret[1]=false;
  for(int i=2; i*i<=m; i++){
    if(ret[i]){
      for(int j=2; i*j<m; j++){
        ret[i*j]=false;
      }
    }
  }
  return ret;
}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}
