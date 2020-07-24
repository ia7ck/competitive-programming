void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  int n, h; rd(n, h);
  auto l=new int[](n), r=new int[](n);
  foreach(i; 0..n) rd(l[i], r[i]);

  int mx=0, len=0;
  for(int i=0, j=0, height=h; i<n; i++){
    while(j<n && height>0){
      mx=max(mx, len+(r[j]-l[j])+height);
      if((++j)<n){
        len+=l[j]-l[j-1];
        height-=l[j]-r[j-1];
      }
    }
    if(i+1<n){
      len-=l[i+1]-l[i];
      height+=l[i+1]-r[i];
    }
  }
  writeln(mx);

}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}
