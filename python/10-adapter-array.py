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


def ways(i, adapters):
    """
    returns a count of the number of ways to get to the adapter at index i from the 3 adapters preceding it.
    """
    if i < 0:
        return 0
    if i == 0:
        return 1

    total_ways = 0

    if i == 1:
        n1 = adapters[i] - adapters[i - 1]
        if n1 <= 3:
            total_ways += 1
        if adapters[i] <= 3:
            total_ways += 1

    elif i == 2:
        n1 = adapters[i] - adapters[i - 1]
        n2 = adapters[i] - adapters[i - 2]
        if n1 <= 3:
            total_ways += 1
        if n2 <= 3:
            total_ways += 1
        if adapters[i] <= 3:
            total_ways += 1

    else:
        n1 = adapters[i] - adapters[i - 1]
        n2 = adapters[i] - adapters[i - 2]
        n3 = adapters[i] - adapters[i - 3]
        if n1 <= 3:
            total_ways += 1
        if n2 <= 3:
            total_ways += 1
        if n3 <= 3:
            total_ways += 1

    return total_ways


def part_two():
    """
    determine the total number of distinct ways you can arrange the adapters to connect the charging outlet to
    your device
    :return:
    """
    adapters = parse_input("../input/10-input.txt")
    # adapters = sorted([16,10,15,5,1,11,7,19,6,12,4])
    adapters = sorted([1,2,3,4])
    print(adapters)
    tot = 0
    for i in range(len(adapters)):
        cur = ways(i, adapters)
        print(f"index {i}  ways {cur}")
        tot += cur
    print(f"total number of distinct ways to arrange adapters: {tot}")




def part_two_plus():
    # jolts = parse_input("../input/10-input.txt")
    jolts = sorted([16,10,15,5,1,11,7,19,6,12,4])
    jolts.insert(0, 0)
    jolts.append(jolts[-1] + 3)
    print(jolts)
    joltMap = {jolts[-2] : 1, jolts[-3] : 1}
    for i in range(len(jolts) - 4, -1, -1):
        combos = joltMap[jolts[i+1]] #You know the next adapter will fit
        if jolts[i+3] - jolts[i] <= 3:
            combos += joltMap[jolts[i+2]] + joltMap[jolts[i+3]]
        elif jolts[i+2] - jolts[i] <= 3:
            combos += joltMap[jolts[i+2]]
        joltMap[jolts[i]] = combos
    print(joltMap[0])


part_two_plus()