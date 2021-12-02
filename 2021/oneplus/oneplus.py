from collections import Counter
import collections
import itertools

with open("input") as f:
    lines = f.readlines()


def sliding(xs, maxlen):
    dq = collections.deque([], maxlen=maxlen)
    for x in xs:
        dq.append(x)
        if len(dq) == maxlen:
            yield list(dq)


int_lines = map(int, lines)
sliding_lines = list(sliding(int_lines, 3))

print(
    Counter(
        map(lambda l: sum(l[1]) > sum(l[0]), zip(sliding_lines, sliding_lines[1:]))
    )[True]
)
