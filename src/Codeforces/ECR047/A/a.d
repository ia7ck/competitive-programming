void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  int n, m; rd(n, m);
  auto c=readln.split.to!(int[]);
  auto a=readln.split.to!(int[]);

  int cnt=0;
  for(int i=0, j=0; i<n && j<m; ){
    if(c[i]<=a[j]){
      cnt++;
      i++; j++;
    }else{
      i++;
    }
  }

  writeln(cnt);
}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}