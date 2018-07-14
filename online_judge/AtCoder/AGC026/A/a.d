void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  int n; rd(n);
  auto a=readln.split.to!(int[]);

  int cur=-1, len=1;
  int need=0;
  foreach(e; a){
    if(cur==e) len++;
    else{
      need+=len/2;
      len=1;
      cur=e;
    }
  }  

  writeln(need+len/2);

}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}