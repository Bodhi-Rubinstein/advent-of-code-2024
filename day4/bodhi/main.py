import pandas as pd

def df_to_matrix(df):
    matrix = []
    for index,row in df.iterrows():
        curr = []
        for i in range(len(row[0])):
            curr.append(row[0][i])
        matrix.append(curr)
    return matrix

def count_xmas(mat):
    count = 0
    rows = len(mat)
    for i in range(len(mat)):  
        cols = len(mat[i])
        for j in range(cols):
            if mat[i][j] == "X":
                #Up
                if (i >= 3 and mat[i-1][j] == "M" and mat[i-2][j] == "A" and mat[i-3][j] == "S"):
                    count += 1
                #Down
                if (i+3 < rows and mat[i+1][j] == "M" and mat[i+2][j] == "A" and mat[i+3][j] == "S"):
                    count += 1
                #Right
                if (j+3 < cols and mat[i][j+1] == "M" and mat[i][j+2] == "A" and mat[i][j+3] == "S"):
                    count += 1
                #Left
                if (j >= 3 and mat[i][j-1] == "M" and mat[i][j-2] == "A" and mat[i][j-3] == "S"):
                    count += 1
                #Up-Right
                if (i >= 3 and j+3 < cols and mat[i-1][j+1] == "M" and mat[i-2][j+2] == "A" and mat[i-3][j+3] == "S"):
                    count += 1
                #Up-Left
                if (i >= 3 and j >= 3 and mat[i-1][j-1] == "M" and mat[i-2][j-2] == "A" and mat[i-3][j-3] == "S"):
                    count += 1
                #Down-Right
                if (i+3 < rows and j+3 < cols and mat[i+1][j+1] == "M" and mat[i+2][j+2] == "A" and mat[i+3][j+3] == "S"):
                    count += 1
                #Down-Left
                if (i+3 < rows and j >= 3 and mat[i+1][j-1] == "M" and mat[i+2][j-2] == "A" and mat[i+3][j-3] == "S"):
                    count += 1
            else:
                continue
    return count

def count_mas_x(mat):
    count = 0
    rows = len(mat)
    for i in range(len(mat)):  
        cols = len(mat[i])
        for j in range(cols):
            if (mat[i][j] == "A" and i > 0 and i+1 < rows and j > 0 and j+1 < cols):
                if (((mat[i-1][j-1] == "M" and mat[i+1][j+1] == "S") or (mat[i-1][j-1] == "S" and mat[i+1][j+1] == "M"))  and  ((mat[i-1][j+1] == "M" and mat[i+1][j-1] == "S") or (mat[i-1][j+1] == "S" and mat[i+1][j-1] == "M"))):
                    count += 1
            else:
                continue
    return count


df = pd.read_csv("/Users/bodhir/Documents/programming/advent-of-code-2024/day4/bodhi/input.txt", header=None)
xmas_matrix = df_to_matrix(df)
print(str(xmas_matrix))
xmas_count = count_xmas(xmas_matrix)
mas_x_count = count_mas_x(xmas_matrix)
print("XMAS Count: " + str(xmas_count))
print("X-MAS Count: " + str(mas_x_count))
