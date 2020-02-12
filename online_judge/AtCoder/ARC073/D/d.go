package main

import "fmt"

func chmax(a *int, b int) {
	if *a < b {
		*a = b
	}
}

func main() {
	var n, W int
	fmt.Scan(&n, &W)
	w := make([]int, n)
	v := make([]int, n)
	for i := 0; i < n; i++ {
		fmt.Scan(&w[i], &v[i])
	}

	sw := n * 3
	dp := make([][]int, n+1)
	for k := 0; k <= n; k++ {
		dp[k] = make([]int, sw+1)
	}
	for i := 0; i < n; i++ {
		for k := n - 1; k >= 0; k-- {
			for u := 0; u+(w[i]-w[0]) <= sw; u++ {
				chmax(&dp[k+1][u+(w[i]-w[0])], dp[k][u]+v[i])
			}
		}
	}
	ans := 0
	for k := 1; k <= n; k++ {
		wmax := W - w[0]*k
		if wmax >= 0 {
			for u := 0; u <= wmax && u <= sw; u++ {
				chmax(&ans, dp[k][u])
			}
		}
	}
	fmt.Println(ans)
}
