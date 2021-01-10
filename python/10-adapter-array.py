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


def connectable_indices(i, adapters):
    """
    returns a list of the indices that can be "connected" to from index i in adapters. This function will check
     indices i + 1, i + 2, and i + 3 to see if they are within a 3 voltage difference from adapters[i]
    """

    indices = []

    if i < len(adapters) - 1 and adapters[i + 1] - adapters[i] <= 3:
        indices.append(i + 1)
    if i < len(adapters) - 2 and adapters[i + 2] - adapters[i] <= 3:
        indices.append(i + 2)
    if i < len(adapters) - 3 and adapters[i + 3] - adapters[i] <= 3:
        indices.append(i + 3)

    return indices


def part_two():
    """
    determine the total number of distinct ways you can arrange the adapters to connect the charging outlet to
    your device
    :return:
    """
    adapters = parse_input("../input/10-input.txt")
    adapters = sorted(adapters)
    # insert a 0 element at the start of adapters to represent the starting connection
    adapters.insert(0, 0)

    # counts is a list that keeps a running count of the total number of ways to connect the adapters at index i
    # to the end of the adapter list.
    # count will be prefilled with zeros, and the last element of count will be defaulted to 1
    counts = [0] * len(adapters)
    counts[-1] = 1

    # work backwards in adapters, starting from the last adapter, and keeping a running count of the number of ways
    # that adapters[i] can be connected to the last adapter. We're starting from the second to last element of adapters
    # since there is always only one way to connect to the last adapter
    for i in range(len(adapters) - 2, -1, -1):
        cis = connectable_indices(i, adapters)
        path_count = sum(map(lambda ci: counts[ci], cis))
        counts[i] = path_count

    print(counts)
    print(f"total number of distinct ways to arrange adapters: {counts[0]}")


part_two()
