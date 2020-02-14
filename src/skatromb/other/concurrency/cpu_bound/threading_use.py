#!/usr/bin/env python3
import concurrent.futures
import time
import logging


def cpu_bound(number):
    logger.info(str(number))
    return sum(i * i for i in range(number))


def find_sums(numbers):
    with concurrent.futures.ThreadPoolExecutor(max_workers=5) as executor:
        executor.map(cpu_bound, numbers)


logging.basicConfig(format='%(asctime)s (UTC): %(levelname)s - %(message)s', level=logging.INFO)
logger = logging.getLogger(__name__)

if __name__ == "__main__":
    numbers = [5_000_000 + x for x in range(20)]

    start_time = time.time()
    find_sums(numbers)
    duration = time.time() - start_time
    print(f"Duration {duration} seconds")
