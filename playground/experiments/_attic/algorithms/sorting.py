"""
Comprehensive Sorting Algorithms Implementation

This module contains implementations of various sorting algorithms
with time and space complexity analysis.
"""

from typing import List, Callable
import time
from functools import wraps


def time_it(func: Callable) -> Callable:
    """Decorator to measure execution time."""
    @wraps(func)
    def wrapper(*args, **kwargs):
        start = time.time()
        result = func(*args, **kwargs)
        end = time.time()
        print(f"{func.__name__} took {(end - start) * 1000:.4f}ms")
        return result
    return wrapper


class SortingAlgorithms:
    """Collection of sorting algorithms."""

    @staticmethod
    def bubble_sort(arr: List[int]) -> List[int]:
        """
        Bubble Sort - O(n²) time, O(1) space

        Repeatedly steps through the list, compares adjacent elements
        and swaps them if they are in wrong order.
        """
        arr = arr.copy()
        n = len(arr)

        for i in range(n):
            swapped = False
            for j in range(0, n - i - 1):
                if arr[j] > arr[j + 1]:
                    arr[j], arr[j + 1] = arr[j + 1], arr[j]
                    swapped = True

            if not swapped:
                break

        return arr

    @staticmethod
    def selection_sort(arr: List[int]) -> List[int]:
        """
        Selection Sort - O(n²) time, O(1) space

        Divides input into sorted and unsorted regions.
        Repeatedly selects the smallest element from unsorted region.
        """
        arr = arr.copy()
        n = len(arr)

        for i in range(n):
            min_idx = i
            for j in range(i + 1, n):
                if arr[j] < arr[min_idx]:
                    min_idx = j

            arr[i], arr[min_idx] = arr[min_idx], arr[i]

        return arr

    @staticmethod
    def insertion_sort(arr: List[int]) -> List[int]:
        """
        Insertion Sort - O(n²) time, O(1) space

        Builds the final sorted array one item at a time.
        Efficient for small data sets and nearly sorted data.
        """
        arr = arr.copy()
        n = len(arr)

        for i in range(1, n):
            key = arr[i]
            j = i - 1

            while j >= 0 and arr[j] > key:
                arr[j + 1] = arr[j]
                j -= 1

            arr[j + 1] = key

        return arr

    @staticmethod
    def merge_sort(arr: List[int]) -> List[int]:
        """
        Merge Sort - O(n log n) time, O(n) space

        Divide and conquer algorithm that divides array into halves,
        recursively sorts them, and merges the sorted halves.
        """
        if len(arr) <= 1:
            return arr

        mid = len(arr) // 2
        left = SortingAlgorithms.merge_sort(arr[:mid])
        right = SortingAlgorithms.merge_sort(arr[mid:])

        return SortingAlgorithms._merge(left, right)

    @staticmethod
    def _merge(left: List[int], right: List[int]) -> List[int]:
        """Merge two sorted arrays."""
        result = []
        i = j = 0

        while i < len(left) and j < len(right):
            if left[i] <= right[j]:
                result.append(left[i])
                i += 1
            else:
                result.append(right[j])
                j += 1

        result.extend(left[i:])
        result.extend(right[j:])

        return result

    @staticmethod
    def quick_sort(arr: List[int]) -> List[int]:
        """
        Quick Sort - O(n log n) average, O(n²) worst, O(log n) space

        Picks a pivot element and partitions array around it.
        Recursively sorts the partitions.
        """
        if len(arr) <= 1:
            return arr

        pivot = arr[len(arr) // 2]
        left = [x for x in arr if x < pivot]
        middle = [x for x in arr if x == pivot]
        right = [x for x in arr if x > pivot]

        return SortingAlgorithms.quick_sort(left) + middle + SortingAlgorithms.quick_sort(right)

    @staticmethod
    def heap_sort(arr: List[int]) -> List[int]:
        """
        Heap Sort - O(n log n) time, O(1) space

        Uses binary heap data structure to sort.
        Builds a max heap and repeatedly extracts maximum.
        """
        arr = arr.copy()
        n = len(arr)

        # Build max heap
        for i in range(n // 2 - 1, -1, -1):
            SortingAlgorithms._heapify(arr, n, i)

        # Extract elements from heap
        for i in range(n - 1, 0, -1):
            arr[0], arr[i] = arr[i], arr[0]
            SortingAlgorithms._heapify(arr, i, 0)

        return arr

    @staticmethod
    def _heapify(arr: List[int], n: int, i: int):
        """Heapify subtree rooted at index i."""
        largest = i
        left = 2 * i + 1
        right = 2 * i + 2

        if left < n and arr[left] > arr[largest]:
            largest = left

        if right < n and arr[right] > arr[largest]:
            largest = right

        if largest != i:
            arr[i], arr[largest] = arr[largest], arr[i]
            SortingAlgorithms._heapify(arr, n, largest)

    @staticmethod
    def counting_sort(arr: List[int]) -> List[int]:
        """
        Counting Sort - O(n + k) time, O(k) space

        Integer sorting algorithm that operates by counting objects.
        Works well when the range of input is not significantly greater than n.
        """
        if not arr:
            return arr

        max_val = max(arr)
        min_val = min(arr)
        range_size = max_val - min_val + 1

        count = [0] * range_size
        output = [0] * len(arr)

        # Count occurrences
        for num in arr:
            count[num - min_val] += 1

        # Calculate cumulative count
        for i in range(1, len(count)):
            count[i] += count[i - 1]

        # Build output array
        for num in reversed(arr):
            output[count[num - min_val] - 1] = num
            count[num - min_val] -= 1

        return output

    @staticmethod
    def radix_sort(arr: List[int]) -> List[int]:
        """
        Radix Sort - O(d * (n + k)) time

        Non-comparative sorting algorithm.
        Sorts integers by processing individual digits.
        """
        if not arr:
            return arr

        max_val = max(arr)
        exp = 1

        while max_val // exp > 0:
            arr = SortingAlgorithms._counting_sort_by_digit(arr, exp)
            exp *= 10

        return arr

    @staticmethod
    def _counting_sort_by_digit(arr: List[int], exp: int) -> List[int]:
        """Helper for radix sort - sorts by specific digit."""
        n = len(arr)
        output = [0] * n
        count = [0] * 10

        # Count occurrences
        for num in arr:
            index = (num // exp) % 10
            count[index] += 1

        # Cumulative count
        for i in range(1, 10):
            count[i] += count[i - 1]

        # Build output
        for i in range(n - 1, -1, -1):
            index = (arr[i] // exp) % 10
            output[count[index] - 1] = arr[i]
            count[index] -= 1

        return output

    @staticmethod
    def bucket_sort(arr: List[float], bucket_count: int = 10) -> List[float]:
        """
        Bucket Sort - O(n + k) average time

        Distributes elements into buckets, sorts buckets individually,
        then concatenates results.
        """
        if not arr:
            return arr

        # Create buckets
        buckets = [[] for _ in range(bucket_count)]

        # Distribute elements
        max_val = max(arr)
        min_val = min(arr)
        range_size = max_val - min_val

        for num in arr:
            if range_size == 0:
                index = 0
            else:
                index = int((num - min_val) / range_size * (bucket_count - 1))
            buckets[index].append(num)

        # Sort buckets and concatenate
        result = []
        for bucket in buckets:
            result.extend(sorted(bucket))

        return result


def demo():
    """Demonstrate all sorting algorithms."""
    import random

    test_cases = [
        ([64, 34, 25, 12, 22, 11, 90], "Small array"),
        ([5, 2, 8, 1, 9], "Small random"),
        (list(range(100, 0, -1)), "Reverse sorted"),
        ([1] * 50, "All same"),
        (random.sample(range(1000), 100), "Random 100 elements")
    ]

    algorithms = [
        ("Bubble Sort", SortingAlgorithms.bubble_sort),
        ("Selection Sort", SortingAlgorithms.selection_sort),
        ("Insertion Sort", SortingAlgorithms.insertion_sort),
        ("Merge Sort", SortingAlgorithms.merge_sort),
        ("Quick Sort", SortingAlgorithms.quick_sort),
        ("Heap Sort", SortingAlgorithms.heap_sort),
        ("Counting Sort", SortingAlgorithms.counting_sort),
        ("Radix Sort", SortingAlgorithms.radix_sort),
    ]

    for arr, description in test_cases:
        print(f"\n{'=' * 60}")
        print(f"Test case: {description}")
        print(f"Input size: {len(arr)}")
        print(f"{'=' * 60}")

        for name, algo in algorithms:
            sorted_arr = algo(arr.copy() if isinstance(arr[0], int) else arr)
            is_sorted = sorted_arr == sorted(arr)
            status = "✓" if is_sorted else "✗"
            print(f"{status} {name}: {'PASSED' if is_sorted else 'FAILED'}")


if __name__ == '__main__':
    demo()
