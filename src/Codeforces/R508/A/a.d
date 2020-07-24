void main(){
  import std.stdio, std.string, std.conv, std.algorithm;

  int n, k; rd(n, k);
  auto s=readln.chomp.to!(char[]);

  int[char] freq;
  foreach(c; s){
    if(c in freq) freq[c]++;
    else freq[c]=1;
  }

  int mn=10^^9;
  for(char c='A'; c<'A'+k; c++){
    if(c in freq) mn=min(mn, freq[c]);
    else mn=min(mn, 0);
  }
  writeln(mn*k);

}

void rd(T...)(ref T x){
  import std.stdio, std.string, std.conv;
  auto l=readln.split;
  assert(l.length==x.length);
  foreach(i, ref e; x) e=l[i].to!(typeof(e));
}
