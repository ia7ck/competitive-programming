void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  int n; rd(n);
  int k=1;
  while((k+1)*(k+1)<=n) k++;
  int[] perm;
  for(int i=1; i<=n; ){
    int[] a;
    for(int j=0; j<k && i<=n; j++){
      a~=(i++);
    }
    perm=a~perm;
  }
  writefln("%(%s %)", perm);
}

/*
  19
  [17 18 19] [13 14 15 16] [9 10 11 12] [5 6 7 8] [1 2 3 4]
*/

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}