"""
Comprehensive Searching Algorithms Implementation

This module contains various searching algorithms and related utilities.
"""

from typing import List, Optional, Tuple
import math


class SearchingAlgorithms:
    """Collection of searching algorithms."""

    @staticmethod
    def linear_search(arr: List[int], target: int) -> int:
        """
        Linear Search - O(n) time, O(1) space

        Sequentially checks each element until target is found.

        Returns:
            Index of target if found, -1 otherwise
        """
        for i, val in enumerate(arr):
            if val == target:
                return i
        return -1

    @staticmethod
    def binary_search(arr: List[int], target: int) -> int:
        """
        Binary Search - O(log n) time, O(1) space

        Searches sorted array by repeatedly dividing search interval in half.
        Array must be sorted.

        Returns:
            Index of target if found, -1 otherwise
        """
        left, right = 0, len(arr) - 1

        while left <= right:
            mid = left + (right - left) // 2

            if arr[mid] == target:
                return mid
            elif arr[mid] < target:
                left = mid + 1
            else:
                right = mid - 1

        return -1

    @staticmethod
    def binary_search_recursive(arr: List[int], target: int,
                                left: int = 0, right: int = None) -> int:
        """
        Binary Search (Recursive) - O(log n) time, O(log n) space

        Recursive implementation of binary search.
        """
        if right is None:
            right = len(arr) - 1

        if left > right:
            return -1

        mid = left + (right - left) // 2

        if arr[mid] == target:
            return mid
        elif arr[mid] < target:
            return SearchingAlgorithms.binary_search_recursive(arr, target, mid + 1, right)
        else:
            return SearchingAlgorithms.binary_search_recursive(arr, target, left, mid - 1)

    @staticmethod
    def jump_search(arr: List[int], target: int) -> int:
        """
        Jump Search - O(√n) time, O(1) space

        Works on sorted arrays. Jumps ahead by fixed steps,
        then performs linear search in identified block.

        Returns:
            Index of target if found, -1 otherwise
        """
        n = len(arr)
        step = int(math.sqrt(n))
        prev = 0

        # Find block where target may exist
        while arr[min(step, n) - 1] < target:
            prev = step
            step += int(math.sqrt(n))
            if prev >= n:
                return -1

        # Linear search in identified block
        while arr[prev] < target:
            prev += 1
            if prev == min(step, n):
                return -1

        if arr[prev] == target:
            return prev

        return -1

    @staticmethod
    def interpolation_search(arr: List[int], target: int) -> int:
        """
        Interpolation Search - O(log log n) average, O(n) worst

        Works on sorted, uniformly distributed arrays.
        Estimates position based on value.

        Returns:
            Index of target if found, -1 otherwise
        """
        left, right = 0, len(arr) - 1

        while left <= right and target >= arr[left] and target <= arr[right]:
            if left == right:
                if arr[left] == target:
                    return left
                return -1

            # Estimate position
            pos = left + int(((target - arr[left]) / (arr[right] - arr[left])) *
                           (right - left))

            if arr[pos] == target:
                return pos
            elif arr[pos] < target:
                left = pos + 1
            else:
                right = pos - 1

        return -1

    @staticmethod
    def exponential_search(arr: List[int], target: int) -> int:
        """
        Exponential Search - O(log n) time

        Finds range where target exists, then performs binary search.
        Useful for unbounded/infinite arrays.

        Returns:
            Index of target if found, -1 otherwise
        """
        if arr[0] == target:
            return 0

        # Find range for binary search
        i = 1
        while i < len(arr) and arr[i] <= target:
            i *= 2

        # Binary search in found range
        left = i // 2
        right = min(i, len(arr) - 1)

        while left <= right:
            mid = left + (right - left) // 2

            if arr[mid] == target:
                return mid
            elif arr[mid] < target:
                left = mid + 1
            else:
                right = mid - 1

        return -1

    @staticmethod
    def ternary_search(arr: List[int], target: int) -> int:
        """
        Ternary Search - O(log₃ n) time

        Divides array into three parts and determines which part
        contains target.

        Returns:
            Index of target if found, -1 otherwise
        """
        left, right = 0, len(arr) - 1

        while left <= right:
            # Divide into three parts
            mid1 = left + (right - left) // 3
            mid2 = right - (right - left) // 3

            if arr[mid1] == target:
                return mid1
            if arr[mid2] == target:
                return mid2

            if target < arr[mid1]:
                right = mid1 - 1
            elif target > arr[mid2]:
                left = mid2 + 1
            else:
                left = mid1 + 1
                right = mid2 - 1

        return -1

    @staticmethod
    def fibonacci_search(arr: List[int], target: int) -> int:
        """
        Fibonacci Search - O(log n) time, O(1) space

        Uses Fibonacci numbers to divide array.
        Similar to binary search but with different division strategy.

        Returns:
            Index of target if found, -1 otherwise
        """
        n = len(arr)

        # Initialize Fibonacci numbers
        fib_m2 = 0  # (m-2)'th Fibonacci number
        fib_m1 = 1  # (m-1)'th Fibonacci number
        fib_m = fib_m2 + fib_m1  # m'th Fibonacci number

        # Find smallest Fibonacci number >= n
        while fib_m < n:
            fib_m2 = fib_m1
            fib_m1 = fib_m
            fib_m = fib_m2 + fib_m1

        offset = -1

        while fib_m > 1:
            # Check if fib_m2 is valid index
            i = min(offset + fib_m2, n - 1)

            if arr[i] < target:
                fib_m = fib_m1
                fib_m1 = fib_m2
                fib_m2 = fib_m - fib_m1
                offset = i
            elif arr[i] > target:
                fib_m = fib_m2
                fib_m1 = fib_m1 - fib_m2
                fib_m2 = fib_m - fib_m1
            else:
                return i

        # Check last element
        if fib_m1 and offset + 1 < n and arr[offset + 1] == target:
            return offset + 1

        return -1

    @staticmethod
    def find_first_occurrence(arr: List[int], target: int) -> int:
        """
        Find first occurrence of target in sorted array with duplicates.

        Returns:
            Index of first occurrence, -1 if not found
        """
        left, right = 0, len(arr) - 1
        result = -1

        while left <= right:
            mid = left + (right - left) // 2

            if arr[mid] == target:
                result = mid
                right = mid - 1  # Continue searching left
            elif arr[mid] < target:
                left = mid + 1
            else:
                right = mid - 1

        return result

    @staticmethod
    def find_last_occurrence(arr: List[int], target: int) -> int:
        """
        Find last occurrence of target in sorted array with duplicates.

        Returns:
            Index of last occurrence, -1 if not found
        """
        left, right = 0, len(arr) - 1
        result = -1

        while left <= right:
            mid = left + (right - left) // 2

            if arr[mid] == target:
                result = mid
                left = mid + 1  # Continue searching right
            elif arr[mid] < target:
                left = mid + 1
            else:
                right = mid - 1

        return result

    @staticmethod
    def count_occurrences(arr: List[int], target: int) -> int:
        """
        Count occurrences of target in sorted array.

        Returns:
            Number of occurrences
        """
        first = SearchingAlgorithms.find_first_occurrence(arr, target)
        if first == -1:
            return 0

        last = SearchingAlgorithms.find_last_occurrence(arr, target)
        return last - first + 1

    @staticmethod
    def find_peak_element(arr: List[int]) -> int:
        """
        Find a peak element (greater than its neighbors).

        Returns:
            Index of a peak element
        """
        left, right = 0, len(arr) - 1

        while left < right:
            mid = left + (right - left) // 2

            if arr[mid] > arr[mid + 1]:
                right = mid
            else:
                left = mid + 1

        return left

    @staticmethod
    def search_rotated_array(arr: List[int], target: int) -> int:
        """
        Search in a rotated sorted array.

        Returns:
            Index of target if found, -1 otherwise
        """
        left, right = 0, len(arr) - 1

        while left <= right:
            mid = left + (right - left) // 2

            if arr[mid] == target:
                return mid

            # Determine which half is sorted
            if arr[left] <= arr[mid]:
                # Left half is sorted
                if arr[left] <= target < arr[mid]:
                    right = mid - 1
                else:
                    left = mid + 1
            else:
                # Right half is sorted
                if arr[mid] < target <= arr[right]:
                    left = mid + 1
                else:
                    right = mid - 1

        return -1


def demo():
    """Demonstrate all searching algorithms."""
    print("=" * 60)
    print("Searching Algorithms Demo")
    print("=" * 60)

    # Test sorted array
    sorted_arr = [1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25]
    target = 15

    algorithms = [
        ("Linear Search", SearchingAlgorithms.linear_search),
        ("Binary Search", SearchingAlgorithms.binary_search),
        ("Binary Search (Recursive)", SearchingAlgorithms.binary_search_recursive),
        ("Jump Search", SearchingAlgorithms.jump_search),
        ("Interpolation Search", SearchingAlgorithms.interpolation_search),
        ("Exponential Search", SearchingAlgorithms.exponential_search),
        ("Ternary Search", SearchingAlgorithms.ternary_search),
        ("Fibonacci Search", SearchingAlgorithms.fibonacci_search),
    ]

    print(f"\nSearching for {target} in: {sorted_arr}\n")

    for name, algo in algorithms:
        result = algo(sorted_arr, target)
        status = "✓" if result == sorted_arr.index(target) else "✗"
        print(f"{status} {name}: Found at index {result}")

    # Test with duplicates
    print("\n" + "=" * 60)
    print("Testing with Duplicates")
    print("=" * 60)

    dup_arr = [1, 2, 2, 2, 3, 4, 5, 5, 5, 5, 6, 7]
    target = 5

    print(f"\nArray: {dup_arr}")
    print(f"Target: {target}")
    print(f"First occurrence: {SearchingAlgorithms.find_first_occurrence(dup_arr, target)}")
    print(f"Last occurrence: {SearchingAlgorithms.find_last_occurrence(dup_arr, target)}")
    print(f"Total occurrences: {SearchingAlgorithms.count_occurrences(dup_arr, target)}")

    # Test peak element
    print("\n" + "=" * 60)
    print("Peak Element")
    print("=" * 60)

    peak_arr = [1, 3, 20, 4, 1, 0]
    peak_idx = SearchingAlgorithms.find_peak_element(peak_arr)
    print(f"Array: {peak_arr}")
    print(f"Peak element at index {peak_idx}: {peak_arr[peak_idx]}")

    # Test rotated array
    print("\n" + "=" * 60)
    print("Rotated Array Search")
    print("=" * 60)

    rotated = [4, 5, 6, 7, 0, 1, 2]
    target = 0
    result = SearchingAlgorithms.search_rotated_array(rotated, target)
    print(f"Array: {rotated}")
    print(f"Target: {target}")
    print(f"Found at index: {result}")


if __name__ == '__main__':
    demo()
