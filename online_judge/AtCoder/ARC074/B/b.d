void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  int n; rd(n);
  auto a=readln.split.to!(long[]);

  import std.container;
  auto minHeap=new BinaryHeap!(Array!long, "a>b");
  long sum=0;
  foreach(i; 0..n){
    sum+=a[i];
    minHeap.insert(a[i]);
  }
  auto red=new long[](n+1);
  red[0]=sum;
  foreach(i; n..(n*2)){
    sum+=a[i];
    minHeap.insert(a[i]);
    sum-=minHeap.front;
    red[i-n+1]=sum;
    minHeap.removeFront;
  }
  auto maxHeap=new BinaryHeap!(Array!long, "a<b");
  sum=0;
  foreach(i; (n*2)..(n*3)){
    sum+=a[i];
    maxHeap.insert(a[i]);
  }
  auto blue=new long[](n+1);
  blue[n]=sum;
  foreach_reverse(i; n..(n*2)){
    sum+=a[i];
    maxHeap.insert(a[i]);
    sum-=maxHeap.front;
    blue[i-n]=sum;
    maxHeap.removeFront;
  }
  import std.range;
  long score=reduce!((r, i)=>max(r, red[i]-blue[i]))(-1000000000000000000, iota(0, (n+1)));
  writeln(score);
}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}
