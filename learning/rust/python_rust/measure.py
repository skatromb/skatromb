import math
import random
from time import time
from rust_count import count_approx_rust


def count_approx_python(items, epsilon=0.5, delta=0.001):
    # Will be used to scale tracked_items upwards:
    p = 1
    # Items we're currently tracking, limited in length:
    tracked_items = set()
    # Max length of tracked_items:
    max_tracked = round(
        ((12 / (epsilon ** 2)) *
         math.log2(8 * len(items) / delta)
         ))
    for item in items:
        tracked_items.discard(item)
        if random.random() < p:
            tracked_items.add(item)
        if len(tracked_items) == max_tracked:
            # Drop tracked values with 50% probability.
            # Every item in tracked_items now implies the
            # final length is twice as large.
            tracked_items = {
                item for item in tracked_items
                if random.random() < 0.5
            }
            p /= 2
            if len(tracked_items) == 0:
                raise RuntimeError(
                    "we got unlucky, no answer"
                )
    return int(round(len(tracked_items) / p))


def timeit(f, times, *args, **kwargs):
    start = time()
    for _ in range(times):
        f(*args, **kwargs)
    print(f.__name__, (time() - start) / times)


WORDS = [str(i) for i in range(100_000)] * 100
random.shuffle(WORDS)
timeit(count_approx_rust, 10, WORDS)
timeit(count_approx_python, 10, WORDS)
