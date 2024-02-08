# A program to batch-compute the number of prime numbers below 
# each of the given numbers in the input set
# an optimized algorithm with a cache is used

import time
from functools import lru_cache

#caching
@lru_cache(maxsize=None)
#optimized algorithm
def is_prime(n):
    sieved = set()  #optimal datatype for optimized search of elements
    factor = 2
    while factor * factor <= n + 1:
        if factor not in sieved:
            for i in range(factor*factor, n + 1, factor):
                sieved.add(i)
        factor += 1
    return n not in sieved

@lru_cache(maxsize=None)
def get_primes(n):
    return [p for p in range(1, n) if is_prime(p)]

if __name__ == 'builtins' or __name__ == '__main__':
    input_data = [10000, 40000, 2000]
    for n in input_data:
        tic = time.time()

        result = get_primes(n)

        toc = time.time()
        print(f'Found {len(result)} primes below {n} calculated in {toc - tic} seconds')