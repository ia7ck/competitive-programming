void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  int n; rd(n);
  auto a=readln.split.to!(long[]).sort!"a>b";
  auto b=readln.split.to!(long[]).sort!"a>b";

  long A=0, B=0;
  foreach(i; 0..(n*2)){
    if(i%2==0){
      if(a.length==0){
        b=b[1..$];
      }else if(b.length==0){
        A+=a[0];
        a=a[1..$];
      }else{
        if(a[0]<b[0]) b=b[1..$];
        else A+=a[0], a=a[1..$];
      }
    }else{
      if(a.length==0){
        B+=b[0];
        b=b[1..$];
      }else if(b.length==0){
        a=a[1..$];
      }else{
        if(b[0]<a[0]) a=a[1..$];
        else B+=b[0], b=b[1..$];
      }
    }
  }
  writeln(A-B);
}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}
