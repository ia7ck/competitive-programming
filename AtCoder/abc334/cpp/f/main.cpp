#include <cstdio>
#include <vector>
#include <cmath>
#include <tuple>
#include "atcoder/lazysegtree"

using namespace std;

#define REP(i, n) for (int i = 0; i < (n); ++i)

struct S
{
    double x;
};

struct F
{
    double c;
};

S op(S l, S r)
{
    return S{min(l.x, r.x)};
}

S e()
{
    return S{1e18};
}

S mapping(F l, S r)
{
    if (r.x == 1e18) {
        return r;
    }
    return S{r.x + l.c};
}

F composition(F l, F r)
{
    return F{l.c + r.c};
}

F id()
{
    return F{0.0};
}

int main()
{
    int n, k;
    scanf("%d%d", &n, &k);
    double sx, sy;
    scanf("%lf%lf", &sx, &sy);
    vector<pair<double, double>> points(n);
    REP(i, n)
    {
        double x, y;
        scanf("%lf%lf", &x, &y);
        points[i] = make_pair(x, y);
    }

    atcoder::lazy_segtree<S, op, e, F, mapping, composition, id> seg(k + n);

    {
        double x, y;
        tie(x, y) = points[0];

        seg.set(k - 1, S{hypot(x - sx, y - sy)});
    }

    REP(i, n - 1)
    {
        double x, y;
        tie(x, y) = points[i];
        double nx, ny;
        tie(nx, ny) = points[i + 1];

        double direct = hypot(nx - x, ny - y);
        double bypass = hypot(x - sx, y - sy) + hypot(nx - sx, ny - sy);
        seg.set((i + 1) + (k - 1), S{seg.prod(i, i + k).x + bypass});
        seg.apply(i + 1, (i + 1) + (k - 1), F{direct});
    }

    double x, y;
    tie(x, y) = points[n - 1];
    double last = hypot(x - sx, y - sy);

    double ans = seg.prod(n - 1, n - 1 + k).x + last;
    printf("%.18lf\n", ans);

    return 0;
}
