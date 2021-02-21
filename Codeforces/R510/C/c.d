void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  int n; rd(n);
  auto a=readln.split.to!(int[]);

  auto count=reduce!((r, e)=>r+(e==0 ? 1 : 0))(0, a);
  auto minus=reduce!((r, e)=>r+(e<0 ? 1 : 0))(0, a);
  int[][] ans;
  if(count==0){
    if(minus%2==0){
      for(int i=1; i<n; i++){
        ans~=[1, i, i+1];
      }
    }else{
      int pos=-1;
      foreach(int i, e; a){
        if(e<0){
          if(pos==-1){
            pos=i;
          }else{
            if(a[pos]<e){
              pos=i;
            }
          }
        }
      }
      writeln(2, " ", pos+1);
      if(pos==0){
        for(int i=2; i<n; i++){
          ans~=[1, i, i+1];
        }
      }else if(pos==n-1){
        for(int i=1; i+1<n; i++){
          ans~=[1, i, i+1];
        }
      }else{
        for(int i=1; i<n; i++){
          if(i+1==pos+1){
            ans~=[1, i, i+2];
            i++;
          }else{
            ans~=[1, i, i+1];
          }
        }
      }
    }
  }else{
    int cnt=0;
    int zeropos=-1;
    if(count>=2){
      int last=-1;
      foreach(int i, e; a){ // 最右の0に寄せる
        if(e==0){
          if(last>=0){
            writeln(1, " ", last+1, " ", i+1);
            cnt++;
          }
          last=i;
          zeropos=i;
        }
      }
    }
    for(int i=n-1; i>=0; i--){
      if(a[i]==0){
        zeropos=i;
        break;
      }
    }
    int pos=-1;
    if(minus&1){
      foreach(int i, e; a){
        if(e<0){
          if(pos==-1){
            pos=i;
          }else{
            if(a[pos]<e){
              pos=i;
            }
          }
        }
      }
      writeln(1, " ", pos+1, " ", zeropos+1);
      cnt++;
    }
    if(cnt+1<n){
      writeln(2, " ", zeropos+1);
    }
    int last=-1;
    foreach(int i, e; a){
      if(e==0 || i==pos || i==zeropos) continue;
      if(last>=0){
        ans~=[1, last+1, i+1];
      }
      last=i;
    }
  }

  foreach(aa; ans){
    writefln("%(%s %)", aa);
  }

}

/*

  - 0なし
    - マイナスが偶数個
      全部かける
    - マイナスが奇数個
      マイナスのもので絶対値が最小のものを削除
      あと全部かける

  - 0あり
    - 複数あり
      0だけ集めて1つにする

    - 1つのみ
      - マイナスが偶数個
        0削除してあとぜんぶかける
      - マイナスが奇数個
        全部かける => 0

 */

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}
