import std.stdio, std.string, std.conv, std.algorithm;

void main(){
  int n, m, q; rd(n, m, q);
  struct T{int l, r, t, idx;}
  T[] seqs;
  foreach(i; 0..m){
    int l, r; rd(l, r);
    seqs~=T(l-1, r-1, 0, i);
  }
  foreach(i; 0..q){
    int l, r; rd(l, r);
    seqs~=T(l-1, r-1, 1, i);
  }
  seqs.sort!((a, b)=>(a.r==b.r ? a.t<b.t : a.r<b.r));
  auto tree=new SegmentTree(n);
  auto ans=new int[](q);
  foreach(seq; seqs){
    if(seq.t==0){
      tree.add(seq.l, 1);
    }else{
      ans[seq.idx]=tree.sum(seq.l, seq.r+1);
    }
  }
  writefln("%(%s\n%)", ans);
}

class SegmentTree{
  int n=1;
  int[] dat;
  this(int N){
    while(n<N) n*=2;
    dat.length=n*2-1;
  }
  void add(int i, int x){
    i+=n-1;
    dat[i]+=x;
    while(i>0){
      i=(i-1)/2;
      dat[i]=dat[i*2+1]+dat[i*2+2];
    }
  }
  int sum(int ql, int qr){
    return sum(ql, qr, 0, 0, n);
  }
  int sum(int ql, int qr, int i, int il, int ir){
    if(qr<=il || ir<=ql) return 0;
    if(ql<=il && ir<=qr) return dat[i];
    int m=(il+ir)/2;
    return sum(ql, qr, i*2+1, il, m)+sum(ql, qr, i*2+2, m, ir);
  }
}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}
