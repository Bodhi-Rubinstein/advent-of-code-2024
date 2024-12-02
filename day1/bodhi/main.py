import pandas

def bubble_sort(array):
    n = len(array)

    for i in range(n):
        swapped = False

        for j in range(0, n-j-1):
            if (array[j] < array[j+1]):
                temp = array[j]
                array[j] = array[j+1]
                array[j+1] = temp
                swapped = True
        if (swapped == False):
            break


inputs = pandas.read_csv('/Users/bodhir/Documents/programming/advent-of-code-2024/day1/bodhi/input.csv', header=None, delimiter=r"\s+")
##print (inputs)

left_list = inputs.iloc[:,0].tolist()
right_list = inputs.iloc[:,1].tolist()
left_list.sort()
right_list.sort()
##print(left_list)
##print(right_list)
sum = 0
for i in range(len(left_list)):
    sum += abs(left_list[i] - right_list[i])
print (sum)

similarity_sum = 0
for i in range(len(left_list)):
    similarity_sum += left_list[i] * right_list.count(left_list[i])
print (similarity_sum)
