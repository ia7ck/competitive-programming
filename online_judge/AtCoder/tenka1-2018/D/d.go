package main

import "fmt"

func main() {
	var n int
	fmt.Scan(&n)
	k := 1
	ok := false
	for ; k*(k-1)/2 <= n; k++ {
		if k*(k-1)/2 == n {
			ok = true
			break
		}
	}
	if !ok {
		fmt.Println("No")
		return
	}

	fmt.Println("Yes")
	fmt.Println(k)
	sets := [][]int{{1}, {1}}
	for q := 3; q <= k; q++ {
		a := (q-1)*(q-2)/2 + 1
		for i := 0; i < len(sets); i++ {
			sets[i] = append(sets[i], i+a)
		}
		ns := []int{}
		for v := a; v <= q*(q-1)/2; v++ {
			ns = append(ns, v)
		}
		sets = append(sets, ns)
	}
	for _, s := range sets {
		fmt.Print(len(s))
		for _, v := range s {
			fmt.Printf(" %d", v)
		}
		fmt.Println("")
	}
}
