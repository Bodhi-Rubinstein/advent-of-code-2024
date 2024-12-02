using CSV
using DataFrames
using StatsBase

df = CSV.read("test.txt", DataFrame, delim = "   ", header = false)
println("Test: ", sum(broadcast(abs, sort(df[!,1]) - sort(df[!,2]))))

df1 = CSV.read("input.txt", DataFrame, delim = "   ", header = false)
println("Input: ", sum(broadcast(abs, sort(df1[!,1]) - sort(df1[!,2]))))

dict = countmap(df[!,2])
s = 0
for i in df[!,1]
    if haskey(dict, i)
        global s += dict[i] * i
    end
end
println("Test: ", s)

dict1 = countmap(df1[!,2])
s1 = 0
for i in df1[!,1]
    if haskey(dict1, i)
        global s1 += dict1[i] * i
    end
end
println("Test: ", s1)