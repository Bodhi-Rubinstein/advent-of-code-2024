import pandas as pd
import numpy as np
import math as math
import os

dirname = os.path.dirname(__file__)

def load_data(file_path): #read csv with pandas and delimit by spaces
    df = pd.read_csv(os.path.join(dirname,file_path), header=None, delimiter=r"\s+", names=range(10))
    return df

def analyze_report_damp(array, skip): # takes the report array and the index to skip
    report = []

    #create report with skipped index
    for i in range (len(array)):
        if (math.isnan(array[i])):
            break
        elif (i==skip):
            continue
        else:
            report.append(array[i])
    
    #find initial inc/dec sign
    sign = np.sign(report[1]-report[0])

    length = len(report)

    #print(str(report) + " " + str(skip))

    for j in range(1,length): #iterate through array
        diff = report[j]-report[j-1] #find diff between curr and prev
        diff_sign = np.sign(diff)
        abs_diff = abs(diff)

        '''
        - The levels are either all increasing or all decreasing.
        - Any two adjacent levels differ by at least one and at most three.
        '''
        if (abs_diff == 0 or abs_diff >= 4 or diff_sign != sign):  
            if (skip < length): #If none of the dampened reports are safe, the full report won't be
                return analyze_report_damp(array,skip+1)
            else:
                return 0
        else:
            if (j==length-1):
                #print(str(report) + " " + str(skip) + "SAFE")
                return 1
            else:
                continue
    return 0

inputs_df = load_data('inputs.csv')

damp_safe_sum = 0

for i in range(len(inputs_df)):
    input = inputs_df.iloc[i,:].tolist()
    damp_safe_sum += analyze_report_damp(input,0)

print("Problem Damper Sum: "+str(damp_safe_sum))

