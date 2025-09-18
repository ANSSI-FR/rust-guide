#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <inttypes.h>

#include "counter.h"

int main(int argc, const char **argv)
{
	if (argc < 2)
	{
		return -1;
	}
	size_t n = (size_t)strtoull(argv[1], NULL, 10);

	Counter *c = counter_create();
	for (size_t i = 0; i < n; i++)
	{
		if (counter_incr(c) != 0)
		{
			printf("overflow\n");
			counter_destroy(c);
			return -1;
		}
	}

	printf("%" PRIu32 "\n", counter_get(c));
	counter_destroy(c);

	return 0;
}