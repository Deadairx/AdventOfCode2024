
# Commands used
- `paste`
- `awk`
- `sort`

## Part 1
`paste` allows you to concatinate each line of given files/input 

``` file1.txt
hello
goodbye
```

``` file2.txt
world
universe
```
running `paste file1.txt file2.txt` would result in 

``` result
hello world
goodbye universe
```
using `<()` I can run sub commands as the input, which I use to sort each column
individually using `awk` and `sort`

after that I'm left with rows I can subtract and sum

## Part 2

This is using one line of `awk`, making use of arrays used like a hash map (`count1[$1]`)
and counting the value using `++`. afterwards, we check if a value is in both 
columns, and calculate them as directed.

