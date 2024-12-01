import sys

def getCount(list: list[int], num):
    try:
        firstInd = list.index(num)
        count = 0
        while True:
            count += 1
            try:
                if list[firstInd + count] != num:
                    break
            except IndexError:
                return count
    except ValueError:
        return 0

    return count

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
    similarity += num * getCount(secondList, num)

print(similarity) # PART TWO
