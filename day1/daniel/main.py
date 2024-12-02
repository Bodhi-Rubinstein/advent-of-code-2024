import sys

with open(sys.argv[1]) as f:
    lines = f.readlines()

lines = list(map(lambda x: x.strip().split("   "), lines))

firstList = sorted([int(x[0]) for x in lines])
secondList = sorted([int(x[1]) for x in lines])

dist = 0
for i in range(len(firstList)):
    dist += abs(firstList[i] - secondList[i])

print(dist) # PART ONE

similarity = 0
for num in firstList:
    similarity += num * secondList.count(num)

print(similarity) # PART TWO
