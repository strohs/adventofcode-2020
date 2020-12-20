# Advent Of Code Day 6 - Custom Customs
#

from functools import reduce


def count_yes_responses(lines):
    """Count the number of unique yes answers within a group"""
    yes_set = [ch for line in lines for ch in line]
    return len(set(yes_set))


def count2(customs):
    """
    counts the yes answers in common between each member of a group
    :param customs: a list of strings, representing customer answers to the custom form
    :return:
    """
    resp = reduce(lambda s1, s2: s1.intersection(s2), [set(iter(line)) for line in customs])
    return len(resp)


def part_one():
    total = 0
    with open("../input/06-input.txt", "r") as f:
        # \n\n is the group separator on linux()
        for group in f.read().split("\n\n"):
            lines = [line for line in group.splitlines()]
            total += count_yes_responses(lines)
    return total


def part_two():
    total = 0
    with open("../input/06-input.txt", "r") as f:
        # \n\n is the group separator on linux()
        for group in f.read().split("\n\n"):
            customs = [line for line in group.splitlines()]
            print(customs)
            print("--------------")
            total += count2(customs)
    return total


# print(f"total 'yes' answers across all groups is {part_one()}")
print(f"number of questions to which everyone answered yes for each group {part_two()}")
