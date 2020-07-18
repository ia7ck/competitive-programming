void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  int n; rd(n);
  auto a=readln.split.to!(int[]);

  struct Pair{
    int val, idx;
  }
  auto data=new Pair[](n);
  foreach(i; 0..n) data[i]=Pair(a[i], i);
  sort!((l, r)=>(l.val==r.val ? l.idx<r.idx : l.val<r.val))(data);
  auto b=a.dup;
  sort(b);
  int cnt=0;
  for(int i=0, j=0; i<n && j<n; i++, j++){
    while(i<n && b[i]<=a[data[j].idx]) i++;
    if(i<n) cnt++;
  }
  writeln(cnt);

}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}

/*

7
10 1 1 1 5 5 3

1 2 3 6 4 5 0

*/