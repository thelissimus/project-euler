package main

import "fmt"

func solution(n int) (sum int) {
	for i := 1; i < n; i++ {
		if i % 3 == 0 || i % 5 == 0 {
			sum += i
		}
	}
	return
}

func main() {
	result := solution(1000)
	fmt.Println(result)
}
