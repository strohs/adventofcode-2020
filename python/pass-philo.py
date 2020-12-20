# Day 2: Password Philosophy
# https://adventofcode.com/2020/day/2
import re

policy_str = r"(?P<min>\d+)-(?P<max>\d+) (?P<chr>\w): (?P<pwd>\w+)"
policy_pat = re.compile(policy_str)


class Policy:

    def __init__(self, min, max, chr, pwd):
        self.min = min
        self.max = max
        self.chr = chr
        self.pwd = pwd

    def __str__(self):
        return f"min:{self.min} max:{self.max} chr:{self.chr} pwd:{self.pwd}"

    def valid_policy(self):
        """

        :return: True if this policy pwd is between min and max
        """
        chr_count = len([c for c in self.pwd if c == self.chr])
        return self.min <= chr_count <= self.max

    def valid_policy_position(self):
        c1 = self.pwd[(self.min - 1):self.min]
        c2 = self.pwd[(self.max - 1):self.max]
        return (c1 == self.chr) ^ (c2 == self.chr)


def parse_line(line):
    m = policy_pat.match(line)
    return Policy(int(m.group('min')), int(m.group('max')), m.group('chr'), m.group('pwd'))


file = open("../input/02-input.txt", "r")

# parse file lines into policy objects
policies = list(map(parse_line, file.read().splitlines()))

valid_policies = [p for p in policies if p.valid_policy_position()]
print("total valid policies {}".format(len(valid_policies)))
