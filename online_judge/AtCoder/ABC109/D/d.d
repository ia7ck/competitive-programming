void main(){
  import std.stdio, std.string, std.conv, std.algorithm;
  import std.array;

  int h, w; rd(h, w);
  auto a=new int[][](h, w);
  foreach(i; 0..h){
    a[i]=readln.split.to!(int[]);
  }

  int py=-1, px=-1;
  int[][] moves;
  foreach(i; 0..h){
    foreach(j; 0..w){
      if(a[i][j]&1){
        if(j+1<w){
          a[i][j]--;
          a[i][j+1]++;
          moves~=[i, j, i, j+1];
        }
      }
    }
  }
  foreach(i; 0..h){
    if(a[i][w-1]&1){
      if(i+1<h){
        a[i][w-1]--;
        a[i+1][w-1]++;
        moves~=[i, w-1, i+1, w-1];
      }
    }
  }
  writeln(moves.length);
  foreach(move; moves) writefln("%(%s %)", move.map!(e=>e+1).array);
  // writeln(a);
}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}
