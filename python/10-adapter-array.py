# Advent of Code - Day 10 - Adapter Array
# https:://adventofcode.com/2020/day/10

# the maximum amount that can differ from one adapter to the next
tolerance = 3

def parse_input(filename):
    """parse input into a list of numbers"""
    with open(filename, "r") as f:
        nums = [int(line) for line in f.read().split("\n")]
        return nums


def joltage_differences(adapters):
    """
    computes voltage differences between the charging outlet, the adapters, and your device
    The differences are stored in a dictionary the maps the amount of difference (an integer), to a list of
    joltages that varied by that amount
    """
    # the maximum amount that a joltage can differ by
    tol = 3
    jdiffs = {}
    prev_adapter = 0
    adapters = sorted(adapters)

    for a in adapters:
        diff = a - prev_adapter
        if diff <= tol:
            if diff not in jdiffs:
                jdiffs[diff] = []
            jdiffs[diff].append(a)
            prev_adapter = a
        else:
            print(f"adapter {a} is greater than tolerance by {diff}")
            return jdiffs

    # add your device's built in adapter which is always 3 higher than the highest adapter
    if 3 not in jdiffs:
        jdiffs[3] = []
    jdiffs[3].append(prev_adapter)
    return jdiffs


def part_one():
    adapters = parse_input("../input/10-input.txt")
    diffs = joltage_differences(adapters)
    diff_product = len(diffs[1]) * len(diffs[3])
    print(f"difference product = {diff_product}")
    for k in diffs.keys():
        print(f"{k}: {diffs[k]}")


# part 2 functions start here

def next_valid_indices(adapters, sidx):
    """
    looks at the next 3 (i.e. tolerance) adapters and returns the indices of the adapters that are within
    the tolerance
    """
    valid = []
    for (i, adapter) in enumerate(adapters[sidx + 1: sidx + 1 + tolerance]):
        if adapter - adapters[sidx] <= tolerance:
            valid.append(sidx + 1 + i)
    return valid


def append_indices(adapters, indices):
    """
    append the list of indices to adapters, but only of the index is not already in adapters. Adapters must
    already be sorted in ascending order
    """
    for i in indices:
        if i not in adapters:
            adapters.append(i)


def part_two():
    """
    determine the total number of distinct ways you can arrange the adapters to connect the charging outlet to
    your device
    :return:
    """
    adapters = parse_input("../input/10-input.txt")
    adapters = sorted([0,16,10,15,5,1,11,7,19,6,12,4, 22])
    # adapters = sorted([0,1,2,3,4,7])
    print(adapters)
    counts = []
    to_visit = []
    next_indices = next_valid_indices(adapters, 0)
    print(f"0 -> {next_indices}")
    counts.append(len(next_indices))
    append_indices(to_visit, next_indices)
    while len(to_visit) > 0:
        nidx = to_visit.pop(0)
        next_indices = next_valid_indices(adapters, nidx)
        print(f"{nidx} -> {next_indices}")
        counts.append(len(next_indices))
        append_indices(to_visit, next_indices)

    print(f"final counts  {counts}")
    print(f"total number of distinct ways to arrange adapters: {sum(counts)}")


part_two()
# 0  (0,1  0,2  0,3)
#  1  (1,2  1,3, 1,4)
#   2  (2,3  2,4)
#    3  (3,4)


# 0,1,2,3,4,7
# 0,1,3,4,7
# 0,1,2,4,7
# 0,2,3,4,7
# 0,1,4,7
# 0,3,4,7
# 0,2,4,7


## 0,1,2,3
# 0    (0,1  0,2  0,3)
#  1   (1,2  1,3)
#   2  (2,3)
#    3 ()
#
# 0,1,2,3
# 0,2,3
# 0,1,3
# 0,3