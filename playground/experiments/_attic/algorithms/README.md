# Algorithm Implementations

Comprehensive collection of classic algorithms with detailed explanations and complexity analysis.

## Contents

### 1. Sorting Algorithms (`sorting.py`)

| Algorithm | Time (Avg) | Time (Worst) | Space | Stable | Notes |
|-----------|------------|--------------|-------|--------|-------|
| Bubble Sort | O(n²) | O(n²) | O(1) | Yes | Simple, good for small/nearly sorted |
| Selection Sort | O(n²) | O(n²) | O(1) | No | Minimizes swaps |
| Insertion Sort | O(n²) | O(n²) | O(1) | Yes | Efficient for small/nearly sorted |
| Merge Sort | O(n log n) | O(n log n) | O(n) | Yes | Divide and conquer, guaranteed O(n log n) |
| Quick Sort | O(n log n) | O(n²) | O(log n) | No | Often fastest in practice |
| Heap Sort | O(n log n) | O(n log n) | O(1) | No | In-place, guaranteed O(n log n) |
| Counting Sort | O(n + k) | O(n + k) | O(k) | Yes | Non-comparison, for integers |
| Radix Sort | O(d(n + k)) | O(d(n + k)) | O(n + k) | Yes | For integers, sorts by digits |
| Bucket Sort | O(n + k) | O(n²) | O(n) | Yes | Good for uniformly distributed data |

**Usage:**
```python
from sorting import SortingAlgorithms

arr = [64, 34, 25, 12, 22, 11, 90]
sorted_arr = SortingAlgorithms.merge_sort(arr)
```

### 2. Searching Algorithms (`searching.py`)

| Algorithm | Time (Avg) | Time (Worst) | Space | Requires Sorted |
|-----------|------------|--------------|-------|-----------------|
| Linear Search | O(n) | O(n) | O(1) | No |
| Binary Search | O(log n) | O(log n) | O(1) | Yes |
| Jump Search | O(√n) | O(√n) | O(1) | Yes |
| Interpolation Search | O(log log n) | O(n) | O(1) | Yes (uniform) |
| Exponential Search | O(log n) | O(log n) | O(1) | Yes |
| Ternary Search | O(log₃ n) | O(log₃ n) | O(1) | Yes |
| Fibonacci Search | O(log n) | O(log n) | O(1) | Yes |

**Special Searches:**
- `find_first_occurrence()`: First index of target
- `find_last_occurrence()`: Last index of target
- `count_occurrences()`: Count of target
- `find_peak_element()`: Local maximum
- `search_rotated_array()`: Search in rotated sorted array

**Usage:**
```python
from searching import SearchingAlgorithms

arr = [1, 3, 5, 7, 9, 11, 13, 15, 17]
index = SearchingAlgorithms.binary_search(arr, 11)
```

### 3. Dynamic Programming (`dynamic_programming.py`)

#### Classic Problems

**Fibonacci Sequence**
- Recursive (exponential)
- Memoization (top-down DP)
- Tabulation (bottom-up DP)
- Space-optimized O(1)

**Longest Common Subsequence (LCS)**
- O(m×n) time, O(m×n) space
- Find longest subsequence common to two strings

**0/1 Knapsack**
- O(n×W) time
- Maximum value with weight constraint
- Space-optimized version available

**Coin Change**
- Minimum coins needed
- Number of ways to make change

**Longest Increasing Subsequence (LIS)**
- O(n²) DP solution
- O(n log n) binary search solution

**Edit Distance (Levenshtein)**
- Minimum operations to transform one string to another
- Supports insert, delete, replace

**Matrix Chain Multiplication**
- O(n³) time
- Minimum scalar multiplications

**Maximum Subarray Sum (Kadane's)**
- O(n) time, O(1) space
- Maximum sum of contiguous subarray

**Subset Sum**
- Check if subset exists with given sum
- O(n×target) time

**Rod Cutting**
- Maximum revenue from cutting rod

**Usage:**
```python
from dynamic_programming import DynamicProgramming

# Fibonacci
fib = DynamicProgramming.fibonacci_tabulation(10)

# Knapsack
weights = [10, 20, 30]
values = [60, 100, 120]
max_value = DynamicProgramming.knapsack_01(weights, values, capacity=50)

# LCS
length = DynamicProgramming.lcs_dp("AGGTAB", "GXTXAYB")
```

## Running the Demos

Each file includes a demo function showcasing all algorithms:

```bash
# Sorting algorithms
python sorting.py

# Searching algorithms
python searching.py

# Dynamic programming
python dynamic_programming.py
```

## Algorithm Selection Guide

### When to Use Each Sorting Algorithm

- **Small arrays (n < 50)**: Insertion Sort
- **Nearly sorted data**: Insertion Sort, Bubble Sort
- **Guaranteed O(n log n)**: Merge Sort, Heap Sort
- **Average case performance**: Quick Sort
- **Limited memory**: Heap Sort (in-place)
- **Stable sort needed**: Merge Sort, Bubble Sort
- **Integer data with small range**: Counting Sort, Radix Sort
- **Uniformly distributed data**: Bucket Sort

### When to Use Each Searching Algorithm

- **Unsorted data**: Linear Search
- **Sorted data (general)**: Binary Search
- **Sorted + uniform distribution**: Interpolation Search
- **Unknown size/infinite array**: Exponential Search
- **Need simplicity**: Binary Search
- **Large sorted array**: Jump Search

### Dynamic Programming Problem Patterns

1. **Optimization**: Maximize/minimize a value
   - Knapsack, Rod Cutting, Matrix Chain

2. **Counting**: Number of ways to achieve something
   - Coin Change (ways), Subset Sum

3. **Decision**: Yes/no feasibility
   - Subset Sum (exists)

4. **Sequence**: Find optimal sequence/subsequence
   - LIS, LCS, Edit Distance

## Complexity Cheat Sheet

### Time Complexity Classes
- O(1): Constant
- O(log n): Logarithmic
- O(n): Linear
- O(n log n): Linearithmic
- O(n²): Quadratic
- O(n³): Cubic
- O(2ⁿ): Exponential
- O(n!): Factorial

### Space Complexity
- O(1): In-place
- O(log n): Recursive stack (balanced tree)
- O(n): Linear extra space
- O(n²): Matrix/2D array

## Testing

Add unit tests for all algorithms:

```python
def test_sorting():
    from sorting import SortingAlgorithms

    arr = [64, 34, 25, 12, 22, 11, 90]
    expected = [11, 12, 22, 25, 34, 64, 90]

    assert SortingAlgorithms.merge_sort(arr) == expected
    assert SortingAlgorithms.quick_sort(arr) == expected
```

## Further Reading

### Books
- "Introduction to Algorithms" (CLRS)
- "Algorithm Design Manual" by Skiena
- "Algorithms" by Sedgewick & Wayne

### Online Resources
- [LeetCode](https://leetcode.com) - Practice problems
- [GeeksforGeeks](https://www.geeksforgeeks.org) - Explanations
- [Visualgo](https://visualgo.net) - Algorithm visualizations
- [Big-O Cheat Sheet](https://www.bigocheatsheet.com)

## Contributing

When adding new algorithms:
1. Include detailed docstrings with complexity analysis
2. Add examples and edge cases
3. Include in demo function
4. Update this README

## Performance Tips

1. **Choose the right algorithm** for your data characteristics
2. **Profile before optimizing** - measure, don't guess
3. **Consider trade-offs** - time vs space, simplicity vs performance
4. **Use built-ins** when appropriate - Python's `sorted()` is highly optimized

## License

MIT License
