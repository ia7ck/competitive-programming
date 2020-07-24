void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  int w; rd(w);
  auto a=new long[][](2, w);
  a[0]=readln.split.to!(long[]);  
  a[1]=readln.split.to!(long[]);

  auto s=new long[](w+1);
  auto sr=new long[][](2, w+1);
  auto sl=new long[][](2, w+1);
  foreach(j; 0..w){
    s[j+1]=s[j];
    if(j%2==0) s[j+1]+=a[0][j]*(j*2)+a[1][j]*(j*2+1);
    else s[j+1]+=a[1][j]*(j*2)+a[0][j]*(j*2+1);
  }
  auto sub=new long[][](2, w+1);
  foreach(i; 0..2)foreach(j; 0..w) sub[i][j+1]=sub[i][j]+a[i][j];
  foreach(i; 0..2)for(int j=w-1; j>=0; j--){
    sr[i][j]=sr[i][j+1];
    sr[i][j]-=(sub[i][w]-sub[i][j+1]);
    sr[i][j]+=a[i][j]*(j*2); // ちょっとサボる    

    sl[i][j]=sl[i][j+1];
    sl[i][j]-=(sub[i][w]-sub[i][j+1]);
    sl[i][j]+=a[i][j]*(w*2-1);
  }
  long mx=0;
  for(int j=0; j<=w; j++){
    if(j%2==0) mx=max(mx, s[j]+sr[0][j]+sl[1][j]);
    else mx=max(mx, s[j]+sr[1][j]+sl[0][j]);
  }
  writeln(mx);
}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}