

def parse_input(filename):
    """parse input into a list of numbers"""
    with open(filename, "r") as f:
        nums = [int(line) for line in f.read().split("\n")]
        return nums


def two_sum(v, sum):
    imap = {}
    res = []
    for num in v:
        target = sum - num
        if target in imap:
            res.append((num, target))
        if num not in imap:
            imap[num] = True
    return res


def valid_pair(pair):
    return pair[0] != pair[1]


def part_one(nums):
    """
    find the first occurrence of a number in nums, that does not have two previous numbers (with in the previous 25
     numbers) that sum to it
    """
    for n in range(25, len(nums)):
        sum_pairs = two_sum(nums[(n - 25):n], nums[n])
        # remove any pairs where pair[0] == pair[1]
        valid_pairs = [pair for pair in sum_pairs if valid_pair(pair)]
        if len(valid_pairs) == 0:
            print(f"{nums[n]} does not have two previous numbers that sum to it")
            return nums[n]
            

nums = parse_input("../input/09-input.txt")
part_one(nums)