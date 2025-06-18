import re
import numpy as np
import pandas as pd
pd.set_option('display.max_colwidth', 100000)

def extract_mult(line):
    #print (line)
    mult = re.findall(r"mul\(\d+,\d+\)",str(line))
    #print (str(mult))
    sum_mult=0
    for i in mult:
        nums = re.findall(r"\d+",i)
        #print (str(nums))
        sum_mult += int(nums[0]) * int(nums[1])
    return sum_mult

def extract_mult_enabled(line, enabled):
    extract = re.findall(r"(mul\(\d+,\d+\)|do\(\)|don\'t\(\))", str(line))

    sum_mult=0
    for i in extract:
        if (i == "do()"):
            enabled=True
            #print("Enabled")
            continue
        elif (i == "don't()"):
            enabled=False
            #print("Disabled")
            continue
        else:
            if (enabled):
                nums = re.findall(r"\d+",i)
                sum_mult += int(nums[0]) * int(nums[1])
                #print (nums[0] + " * " + nums[1])
            else:
                continue
    return sum_mult,enabled

total = 0
total_enabled = 0
enabled=True
memory_df = pd.read_csv("/Users/bodhir/Documents/programming/advent-of-code-2024/day3/bodhi/input.txt", header=None, sep='\t')
for i in range(len(memory_df)):
    input = memory_df.iloc[i,:]
    #print (input)
    line_sum = extract_mult(input)
    line_sum_enabled = extract_mult_enabled(input, enabled)
    total += line_sum
    total_enabled += line_sum_enabled[0]
    enabled=line_sum_enabled[1]
print("Multiplication Total: " + str(total))
print("Enabled Multiplication Total: " + str(total_enabled))