void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  long c, d; rd(c, d);
  long l=140, r=170;

  long num=0;
  while(l<=d){
    if(r<=c || d<=l){
      // 
    }else if(l<=c && d<=r){
      num+=d-c;
    }else{
      num+=min(r, d)-max(c, l);
    }
    l*=2; r*=2;
  }
  writeln(num);
}

/*

  [140, 170)
  [280, 340)
  [560, 680)

*/

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}