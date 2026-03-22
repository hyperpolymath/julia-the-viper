"""
Dynamic Programming Algorithms

Classic DP problems with multiple implementation approaches:
- Memoization (top-down)
- Tabulation (bottom-up)
"""

from typing import List, Dict, Tuple
from functools import lru_cache


class DynamicProgramming:
    """Collection of dynamic programming algorithms."""

    # FIBONACCI SEQUENCE

    @staticmethod
    def fibonacci_recursive(n: int) -> int:
        """
        Fibonacci - Naive Recursive - O(2^n) time, O(n) space

        Exponential time complexity - inefficient for large n.
        """
        if n <= 1:
            return n
        return DynamicProgramming.fibonacci_recursive(n - 1) + \
               DynamicProgramming.fibonacci_recursive(n - 2)

    @staticmethod
    def fibonacci_memoization(n: int, memo: Dict[int, int] = None) -> int:
        """
        Fibonacci - Memoization - O(n) time, O(n) space

        Top-down approach with caching.
        """
        if memo is None:
            memo = {}

        if n in memo:
            return memo[n]

        if n <= 1:
            return n

        memo[n] = DynamicProgramming.fibonacci_memoization(n - 1, memo) + \
                  DynamicProgramming.fibonacci_memoization(n - 2, memo)
        return memo[n]

    @staticmethod
    @lru_cache(maxsize=None)
    def fibonacci_lru(n: int) -> int:
        """
        Fibonacci - Using functools.lru_cache decorator.
        """
        if n <= 1:
            return n
        return DynamicProgramming.fibonacci_lru(n - 1) + \
               DynamicProgramming.fibonacci_lru(n - 2)

    @staticmethod
    def fibonacci_tabulation(n: int) -> int:
        """
        Fibonacci - Tabulation - O(n) time, O(n) space

        Bottom-up approach building from base cases.
        """
        if n <= 1:
            return n

        dp = [0] * (n + 1)
        dp[1] = 1

        for i in range(2, n + 1):
            dp[i] = dp[i - 1] + dp[i - 2]

        return dp[n]

    @staticmethod
    def fibonacci_optimized(n: int) -> int:
        """
        Fibonacci - Space Optimized - O(n) time, O(1) space

        Only keeps track of last two values.
        """
        if n <= 1:
            return n

        prev2, prev1 = 0, 1

        for _ in range(2, n + 1):
            curr = prev1 + prev2
            prev2, prev1 = prev1, curr

        return prev1

    # LONGEST COMMON SUBSEQUENCE

    @staticmethod
    def lcs_recursive(s1: str, s2: str, m: int = None, n: int = None) -> int:
        """
        LCS - Recursive - O(2^n) time

        Find length of longest common subsequence.
        """
        if m is None:
            m = len(s1)
        if n is None:
            n = len(s2)

        if m == 0 or n == 0:
            return 0

        if s1[m - 1] == s2[n - 1]:
            return 1 + DynamicProgramming.lcs_recursive(s1, s2, m - 1, n - 1)
        else:
            return max(DynamicProgramming.lcs_recursive(s1, s2, m - 1, n),
                      DynamicProgramming.lcs_recursive(s1, s2, m, n - 1))

    @staticmethod
    def lcs_dp(s1: str, s2: str) -> int:
        """
        LCS - Dynamic Programming - O(m*n) time, O(m*n) space
        """
        m, n = len(s1), len(s2)
        dp = [[0] * (n + 1) for _ in range(m + 1)]

        for i in range(1, m + 1):
            for j in range(1, n + 1):
                if s1[i - 1] == s2[j - 1]:
                    dp[i][j] = 1 + dp[i - 1][j - 1]
                else:
                    dp[i][j] = max(dp[i - 1][j], dp[i][j - 1])

        return dp[m][n]

    # KNAPSACK PROBLEM

    @staticmethod
    def knapsack_01(weights: List[int], values: List[int], capacity: int) -> int:
        """
        0/1 Knapsack - O(n*W) time, O(n*W) space

        Find maximum value that can be obtained with given capacity.
        """
        n = len(weights)
        dp = [[0] * (capacity + 1) for _ in range(n + 1)]

        for i in range(1, n + 1):
            for w in range(1, capacity + 1):
                if weights[i - 1] <= w:
                    dp[i][w] = max(
                        values[i - 1] + dp[i - 1][w - weights[i - 1]],
                        dp[i - 1][w]
                    )
                else:
                    dp[i][w] = dp[i - 1][w]

        return dp[n][capacity]

    @staticmethod
    def knapsack_01_optimized(weights: List[int], values: List[int],
                             capacity: int) -> int:
        """
        0/1 Knapsack - Space Optimized - O(n*W) time, O(W) space
        """
        dp = [0] * (capacity + 1)

        for i in range(len(weights)):
            for w in range(capacity, weights[i] - 1, -1):
                dp[w] = max(dp[w], values[i] + dp[w - weights[i]])

        return dp[capacity]

    # COIN CHANGE

    @staticmethod
    def coin_change_min_coins(coins: List[int], amount: int) -> int:
        """
        Coin Change - Minimum Coins - O(amount * n) time

        Find minimum number of coins to make amount.
        Returns -1 if not possible.
        """
        dp = [float('inf')] * (amount + 1)
        dp[0] = 0

        for coin in coins:
            for i in range(coin, amount + 1):
                dp[i] = min(dp[i], dp[i - coin] + 1)

        return dp[amount] if dp[amount] != float('inf') else -1

    @staticmethod
    def coin_change_ways(coins: List[int], amount: int) -> int:
        """
        Coin Change - Count Ways - O(amount * n) time

        Count number of ways to make amount.
        """
        dp = [0] * (amount + 1)
        dp[0] = 1

        for coin in coins:
            for i in range(coin, amount + 1):
                dp[i] += dp[i - coin]

        return dp[amount]

    # LONGEST INCREASING SUBSEQUENCE

    @staticmethod
    def lis_dp(arr: List[int]) -> int:
        """
        Longest Increasing Subsequence - O(n²) time, O(n) space

        Find length of longest strictly increasing subsequence.
        """
        if not arr:
            return 0

        n = len(arr)
        dp = [1] * n

        for i in range(1, n):
            for j in range(i):
                if arr[j] < arr[i]:
                    dp[i] = max(dp[i], dp[j] + 1)

        return max(dp)

    @staticmethod
    def lis_binary_search(arr: List[int]) -> int:
        """
        LIS - Binary Search Approach - O(n log n) time, O(n) space

        More efficient approach using binary search.
        """
        if not arr:
            return 0

        tails = []

        for num in arr:
            left, right = 0, len(tails)

            while left < right:
                mid = (left + right) // 2
                if tails[mid] < num:
                    left = mid + 1
                else:
                    right = mid

            if left == len(tails):
                tails.append(num)
            else:
                tails[left] = num

        return len(tails)

    # EDIT DISTANCE

    @staticmethod
    def edit_distance(s1: str, s2: str) -> int:
        """
        Edit Distance (Levenshtein) - O(m*n) time, O(m*n) space

        Minimum operations (insert, delete, replace) to convert s1 to s2.
        """
        m, n = len(s1), len(s2)
        dp = [[0] * (n + 1) for _ in range(m + 1)]

        # Base cases
        for i in range(m + 1):
            dp[i][0] = i
        for j in range(n + 1):
            dp[0][j] = j

        for i in range(1, m + 1):
            for j in range(1, n + 1):
                if s1[i - 1] == s2[j - 1]:
                    dp[i][j] = dp[i - 1][j - 1]
                else:
                    dp[i][j] = 1 + min(
                        dp[i - 1][j],      # Delete
                        dp[i][j - 1],      # Insert
                        dp[i - 1][j - 1]   # Replace
                    )

        return dp[m][n]

    # MATRIX CHAIN MULTIPLICATION

    @staticmethod
    def matrix_chain_order(dims: List[int]) -> int:
        """
        Matrix Chain Multiplication - O(n³) time, O(n²) space

        Find minimum scalar multiplications needed.
        dims[i-1] x dims[i] is dimension of matrix i.
        """
        n = len(dims) - 1
        dp = [[0] * n for _ in range(n)]

        for length in range(2, n + 1):
            for i in range(n - length + 1):
                j = i + length - 1
                dp[i][j] = float('inf')

                for k in range(i, j):
                    cost = (dp[i][k] + dp[k + 1][j] +
                           dims[i] * dims[k + 1] * dims[j + 1])
                    dp[i][j] = min(dp[i][j], cost)

        return dp[0][n - 1]

    # MAXIMUM SUBARRAY SUM

    @staticmethod
    def max_subarray_sum(arr: List[int]) -> int:
        """
        Maximum Subarray Sum (Kadane's Algorithm) - O(n) time, O(1) space

        Find maximum sum of contiguous subarray.
        """
        if not arr:
            return 0

        max_sum = curr_sum = arr[0]

        for num in arr[1:]:
            curr_sum = max(num, curr_sum + num)
            max_sum = max(max_sum, curr_sum)

        return max_sum

    # SUBSET SUM

    @staticmethod
    def subset_sum(arr: List[int], target: int) -> bool:
        """
        Subset Sum - O(n * target) time, O(target) space

        Check if there's a subset with given sum.
        """
        dp = [False] * (target + 1)
        dp[0] = True

        for num in arr:
            for i in range(target, num - 1, -1):
                dp[i] = dp[i] or dp[i - num]

        return dp[target]

    # ROD CUTTING

    @staticmethod
    def rod_cutting(prices: List[int], length: int) -> int:
        """
        Rod Cutting - O(n²) time, O(n) space

        Find maximum revenue from cutting rod of given length.
        prices[i] is price of rod of length i+1.
        """
        dp = [0] * (length + 1)

        for i in range(1, length + 1):
            for j in range(i):
                dp[i] = max(dp[i], prices[j] + dp[i - j - 1])

        return dp[length]


def demo():
    """Demonstrate dynamic programming algorithms."""
    print("=" * 60)
    print("Dynamic Programming Algorithms Demo")
    print("=" * 60)

    # Fibonacci
    print("\n--- Fibonacci Sequence ---")
    n = 10
    print(f"F({n}) = {DynamicProgramming.fibonacci_tabulation(n)}")
    print(f"F({n}) optimized = {DynamicProgramming.fibonacci_optimized(n)}")

    # LCS
    print("\n--- Longest Common Subsequence ---")
    s1, s2 = "AGGTAB", "GXTXAYB"
    print(f"LCS('{s1}', '{s2}') = {DynamicProgramming.lcs_dp(s1, s2)}")

    # Knapsack
    print("\n--- 0/1 Knapsack ---")
    weights = [10, 20, 30]
    values = [60, 100, 120]
    capacity = 50
    result = DynamicProgramming.knapsack_01(weights, values, capacity)
    print(f"Weights: {weights}")
    print(f"Values: {values}")
    print(f"Capacity: {capacity}")
    print(f"Maximum value: {result}")

    # Coin Change
    print("\n--- Coin Change ---")
    coins = [1, 2, 5]
    amount = 11
    print(f"Coins: {coins}")
    print(f"Amount: {amount}")
    print(f"Minimum coins: {DynamicProgramming.coin_change_min_coins(coins, amount)}")
    print(f"Number of ways: {DynamicProgramming.coin_change_ways(coins, amount)}")

    # LIS
    print("\n--- Longest Increasing Subsequence ---")
    arr = [10, 9, 2, 5, 3, 7, 101, 18]
    print(f"Array: {arr}")
    print(f"LIS length: {DynamicProgramming.lis_dp(arr)}")
    print(f"LIS length (optimized): {DynamicProgramming.lis_binary_search(arr)}")

    # Edit Distance
    print("\n--- Edit Distance ---")
    s1, s2 = "sunday", "saturday"
    print(f"'{s1}' → '{s2}'")
    print(f"Edit distance: {DynamicProgramming.edit_distance(s1, s2)}")

    # Maximum Subarray
    print("\n--- Maximum Subarray Sum ---")
    arr = [-2, 1, -3, 4, -1, 2, 1, -5, 4]
    print(f"Array: {arr}")
    print(f"Max sum: {DynamicProgramming.max_subarray_sum(arr)}")

    # Subset Sum
    print("\n--- Subset Sum ---")
    arr = [3, 34, 4, 12, 5, 2]
    target = 9
    print(f"Array: {arr}")
    print(f"Target: {target}")
    print(f"Possible: {DynamicProgramming.subset_sum(arr, target)}")


if __name__ == '__main__':
    demo()
