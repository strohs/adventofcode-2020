# Day 1 - Advent of Code
# https://adventofcode.com/2020/day/1

def two_sum(ls, target_sum):
    """
    find all pairs of numbers in ls that sum to the target sum
    :param ls:
    :param target_sum:
    :return: a list of 2-tuples
    """
    pairs = []
    table = {}
    for n in ls:
        target = target_sum - n
        if target in table:
            pairs.append((n, target))
        if target not in table:
            table[n] = 1
    return pairs


def three_sum(ls, target_sum):
    triples = []
    for i in range(0, len(ls)):
        target = target_sum - ls[i]
        pairs = two_sum(ls[i+1:], target)
        for pair in pairs:
            triples.append((ls[i], pair[0], pair[1]))
    return triples


file = open("../input/01-input.txt", "r")

nums = list(map(int, file.read().splitlines()))

# pairs = two_sum(nums, 2020)
# print(pairs)
# print("product is", pairs[0][0] * pairs[0][1])

triples = three_sum(nums, 2020)
print(triples)

file.close()
