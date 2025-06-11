import pandas as pd
import numpy as np
import math as math

def analyze_report(array, safe):
    sign = np.sign(array[1] - array[0])
    n=0
    #handles different columns numbers
    for i in range (len(array)):
        if (math.isnan(array[i])):
            break
        else:
            n+=1

    unsafe = 0
    last = -1
    for j in range(1,n):
        curr = array[j]
        dif = array[j] - array[j-1]
        abs_dif = abs(dif)
        dif_sign = np.sign(dif)
        ##print("j = " + str(j) + ", dif = " + str(dif) +", abs_dif = " + str(abs_dif) +", dif_sign = " + str(dif_sign) +", sign = " + str(sign)+ ", n = " + str(n))
        if (abs_dif >= 4 or abs_dif == 0 or dif_sign != sign):
            if (safe == 0):
                break
            #elif(dif_sign != sign):
            elif(np.sign(array[j]-array[j-2]) == np.sign(array[j+1]-array[j]) or np.sign(array[j+1]-array[j]) == np.sign(array[j+2]-array[j+1]) or np.sign(array[j]-array[j-2]) == np.sign(array[j]-array[j-3])):
                del array[j-1]
                return analyze_report(array, safe-1)
            else:
                ##print(array)
                del array[j]
                ##print(array)
                return analyze_report(array, safe-1)
        else:
            sign = dif_sign
        if (j == (n-1)):
            return 1

    return 0
            
                
inputs_df = pd.read_csv("/Users/bodhir/Documents/programming/advent-of-code-2024/day2/bodhi/inputs.csv", header=None, delimiter=r"\s+", names=range(10))
safe_sum = 0
prob_damp = 0
##print(inputs_df)
for i in range(len(inputs_df)):
    input = inputs_df.iloc[i,:].tolist()
    ##print(input)
    safe_sum += analyze_report(input, 0) 
    prob_damp += analyze_report(input, 1)
print("Original Sum: " + str(safe_sum))
print("Problem Dampner Sum: " + str(prob_damp))
