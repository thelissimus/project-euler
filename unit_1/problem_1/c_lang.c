#include <stdio.h>

int solution(int n)
{
	int sum = 0;

	for (int i = 1; i < n; i++) {
		if (i % 3 == 0 || i % 5 == 0) {
			sum += i;
		}
	}

	return sum;
}

int main(void)
{
	int result = solution(1000);
	printf("%d\n", result);
}
