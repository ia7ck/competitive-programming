// dmd queue.d -of=queue.out -main -unittest -cov
// ./queue.out

class Queue(T) {
private:
  int n, l = 0, r = 0;
  T[] arr;

public:
  this(int size) {
    n = size + 1;
    arr.length = n;
  }

  bool empty() {
    return l == r;
  }

  bool full() {
    return l == (r + 1) % n;
  }

  T front() {
    return arr[l];
  }

  void insertBack(T x) {
    assert(full == false);
    arr[r] = x;
    (r += 1) %= n;
  }

  void removeFront() {
    assert(empty == false);
    (l += 1) %= n;
  }

  unittest {
    auto q = new Queue!(int)(100);
    assert(q.empty);
    foreach (i; 0 .. 100) {
      q.insertBack(i);
    }
    assert(q.full);
    foreach (i; 0 .. 100) {
      assert(q.front == i);
      q.removeFront;
    }
    assert(q.empty);
  }
}

/+
  https://joi2014yo.contest.atcoder.jp/submissions/1966360
  https://joi2014yo.contest.atcoder.jp/tasks/joi2014yo_e
+/
