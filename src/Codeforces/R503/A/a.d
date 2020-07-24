void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  int n, h, a, b, k; rd(n, h, a, b, k);
  struct T{long t, f;}
  import std.math;
  while(k--){
    auto data=new T[](2);
    rd(data[0].t, data[0].f, data[1].t, data[1].f);
    data.sort!"a.t==b.t ? a.f<b.f : a.t<b.t";
    if(data[0].t==data[1].t){
      writeln(abs(data[0].f-data[1].f));
    }else{
      long d=0;
      d+=(data[1].t-data[0].t);
      if(data[0].f<a){
        d+=(a-data[0].f);
        d+=abs(a-data[1].f);
      }else if(data[0].f>b){
        d+=(data[0].f-b);
        d+=abs(b-data[1].f);
      }else{
        d+=abs(data[0].f-data[1].f);
      }
      writeln(d);
    }
  }
}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}
