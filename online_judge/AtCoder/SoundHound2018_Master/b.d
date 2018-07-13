void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  auto s=readln.chomp.to!(char[]);
  int w; rd(w);

  auto t="".to!(char[]);
  int i=0;
  while(i<s.length){
    t~=s[i];
    i+=w;
  }

  writeln(t);

}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}