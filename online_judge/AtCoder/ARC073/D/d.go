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
		for c := n - 1; c >= 0; c-- {
			for s := 0; s+(w[i]-w[0]) <= sw; s++ {
				chmax(&dp[c+1][s+(w[i]-w[0])], dp[c][s]+v[i])
			}
		}
	}
	ans := 0
	for c := 1; c <= n; c++ {
		wmax := W - w[0]*c
		for s := 0; s <= wmax && s <= sw; s++ {
			chmax(&ans, dp[c][s])
		}
	}
	fmt.Println(ans)
}
