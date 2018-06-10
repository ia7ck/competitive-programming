## 概要
LongestIncreasingSubsequence（最長増加部分列）とは、数列から順番を保って要素をいくつか取り出したとき、単調増加になっているもののうち最長のもの。

狭義単調だったり広義単調だったりする。連続した区間でなくてもよいことに注意。

## O(n^2)
dp[i]:=the length of LIS which ends with i-th element とする。境界条件は空の数列はLISになっていて、長さが０であること。遷移は、a[i]を追加できるようなLISのうち最長のものの末尾にくっつける感じでやる。式で書くとdp[i]=max(dp[j])+1；ただし、maxはa[j]<a[i]なるj in [1, i)全てに対して取る。もし、このようなjが存在しなければdp[i]=1；対応するLISは{a[i]}

## O(nlogn) (SegmentTree)
dp配列の定義は上と同じで、遷移を少し変える。具体的にはa[i]の小さい順にdp[i]を埋めていくようにする。これによってdp[i]=max(dp[1, i))+1とできるようになった。区間max・点更新のSegmentTreeがあれば全体でO(nlogn)で済む。

## O(nlogn) (lowerBound)
dp[i]:=長さiのLISの末尾で最小のもの とする。これは、同じ長さのLISなら末尾の値が小さいほうが未来に可能性を残せるだろう、という気持ちから来ている。初期条件はdp[\*]=inf。遷移は、dp[j]<a[i]なるj in [1, i)のうち最大のものに対して、dp[j+1]=a[i]とする。このようなjが存在しなければdp[1]=a[i]。ところで、dp配列は単調増加になっているので各iに対してjを求めるパートでは二分探索（lowerBound）が使えて、全体でO(nlogn)に収まる。

## 例題
https://beta.atcoder.jp/contests/abc038/tasks/abc038_d
