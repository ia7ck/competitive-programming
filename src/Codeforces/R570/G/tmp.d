void main() {
  import std.stdio, std.string, std.conv, std.algorithm;
  import std.container.rbtree;

  auto rbt = new RedBlackTree!(int, "a>b", true);
  rbt.insert(1);
  rbt.insert(1);
  rbt.insert(2);
  rbt.insert(2);
  rbt.insert(3);

  writeln(rbt.front);
  rbt.removeFront;
  rbt.removeKey(2);
  writeln(rbt.front);

}

void rd(T...)(ref T x) {
  import std.stdio : readln;
  import std.string : split;
  import std.conv : to;

  auto l = readln.split;
  assert(l.length == x.length);
  foreach (i, ref e; x)
    e = l[i].to!(typeof(e));
}
