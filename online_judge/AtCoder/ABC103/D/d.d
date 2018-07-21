void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  int n, m; rd(n, m);
  struct Seq{int l, r;}
  auto seqs=new Seq[](m);
  foreach(i; 0..m) rd(seqs[i].l, seqs[i].r);

  sort!((a, b)=>(a.r==b.r ? a.l<b.l : a.r<b.r))(seqs);
  int last=0, need=0; // last -> last+1
  foreach(seq; seqs){
    if(seq.l<=last && last<=seq.r) continue;
    last=seq.r-1;
    need++;
  }
  writeln(need);

}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}