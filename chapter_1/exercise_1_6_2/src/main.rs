/* This coding tasks of Exercise 1.6.2 are:
- Task #1: Modify the binary search algorithm to find the first occurrence of a
number in a sorted array with duplicates. 

- Task #2: The following function (procedure 1.14) calculates the sum of 
all pairs in an array. Optimize it to improve its time complexity.
procedure 1.14: 
procedure SUMOFPAIRS ( arr )
    sum ← 0
    for i ← 0 to length ( arr ) - 1 do
        for j ← i + 1 to length ( arr ) - 1 do
            sum ← sum + arr [ i ] + arr [ j ]
        end for
    end for
    return sum
end procedure

- Task #3: Implement a function to reverse a string using only O(1) extra space.

- Task #4: Implement a function that finds the kth largest element in an unsorted array.
Try to optimize for both time and space complexity.
*/

// Task #1: Modified Binary Search algorithm (finds first occurrence of duplicate number)

fn main() {
    println!("Hello World!");
}

fn binary_search_for_first_occurrence(arr: &[i32], target: i32) -> Option<usize>{
    let mut low: usize = 0;
    let mut high: usize = arr.len();
    let mut first_occurrence: Option<usize> = None;

    while low < high {
        let mid: usize = low + (high - low) / 2;
        if arr[mid] == target {
            first_occurrence = Some(mid);
            high = mid;
        } else if arr[mid] < target {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        first_occurrence
}

/* Task #2: Improve this pseudocode to improve it's time complexity. 
pseudo code: 
procedure 1.14: 
procedure SUMOFPAIRS ( arr )
    sum ← 0
    for i ← 0 to length ( arr ) - 1 do
        for j ← i + 1 to length ( arr ) - 1 do
            sum ← sum + arr [ i ] + arr [ j ]
        end for
    end for
    return sum
end procedure 
*/

/* Here is the result of the Google Gemini show thinking feature when prompted to answer Task #2:
I need to get the relevant content to answer the user's question.

The user wants to optimize the given pseudocode function SUMOFPAIRS and then rewrite the optimized logic in Rust pseudocode.

The original function has a time complexity of O(n^2) because of the nested loops. It calculates the sum of all unique pairs in an array.

To optimize this, we can realize that each element arr[i] will be paired with every subsequent element arr[j] where j > i.  Instead of calculating each pair sum individually, we can iterate through the array once and keep track of the running sum of the elements encountered so far.

Let's consider an example: arr = [a, b, c, d]

Pairs are: (a, b), (a, c), (a, d), (b, c), (b, d), (c, d)

Sum = a + b + a + c + a + d + b + c + b + d + c + d
Sum = 3a + 2b + c + 2b + 2c + d + c + d
Sum = 3a + 4b + 4c + 2d  -- This doesn't look right. Let's rethink.

Let's look at how many times each element appears in the sum of pairs:

a is paired with b, c, d (3 times)
b is paired with c, d (2 times)
c is paired with d (1 time)
Consider the sum of all elements encountered so far as we iterate.
When we are at index i, the element arr[i] will be paired with all the elements that come after it.

Let's try a different approach. Calculate the total sum of all elements. Then, for each element arr[i], we know it will be paired with all subsequent elements.

Total sum of pairs = sum of (arr[i] + arr[j]) for all i < j

Let's iterate through the array. When we are at index i, arr[i] will be added to sum for every j > i. There are n - 1 - i such j values.

Consider the example arr = [a, b, c]
Pairs: (a, b), (a, c), (b, c)
Sum = a + b + a + c + b + c = 2a + 2b + 2c  -- Still not right.

Let's go back to the definition: sum of all pairs.

arr = [a, b, c]
Pairs: (a, b), (a, c), (b, c)
Sum = a + b + a + c + b + c

Let's think about the contribution of each element.
When we consider arr[i], it will be paired with arr[i+1], arr[i+2], ..., arr[n-1].
So, arr[i] will be added n - 1 - i times to the total sum (when it's the first element of the pair) and also i times when it's the second element of a pair formed with a preceding element.

Let's try another approach.
Calculate the sum of all elements in the array. Let this be S.
Consider the sum of pairs: arr[0] + arr[1], arr[0] + arr[2], ..., arr[0] + arr[n-1]
arr[1] + arr[2], ..., arr[1] + arr[n-1]
...
arr[n-2] + arr[n-1]

Let's iterate through the array. For each element arr[i], it will be part of n - 1 - i pairs where it is the first element.

Optimized approach:
Initialize total_sum_of_pairs to 0.
Initialize current_sum to 0.
Iterate through the array with index i from 0 to length(arr) - 1.
In each iteration, arr[i] will form a pair with all the elements after it.
The number of elements after arr[i] is length(arr) - 1 - i.
So, arr[i] will be added length(arr) - 1 - i times.

Let's try again with arr = [a, b, c].
i = 0, element = a, remaining elements = b, c. Pairs: (a, b), (a, c). a is added 2 times.
i = 1, element = b, remaining elements = c. Pairs: (b, c). b is added 1 time.
i = 2, element = c, remaining elements = none. c is added 0 times.

This is not the sum of pairs, but rather the sum of the first element of each pair.

Let's go back to the original formula: sum ← sum + arr [ i ] + arr [ j ]

Consider the contribution of each element to the final sum.
For arr[0], it's added with arr[1], arr[2], ..., arr[n-1] (n-1 times).
For arr[1], it's added with arr[2], ..., arr[n-1] (n-2 times) and also was added with arr[0] in the previous iterations.

Let's think about the total sum of elements.
S = arr[0] + arr[1] + ... + arr[n-1]

Consider the sum of pairs again:
(a+b) + (a+c) + (b+c) for [a, b, c]

Optimized approach:
Initialize total_sum to 0.
Iterate through the array with index i from 0 to length(arr) - 1.
Iterate through the array with index j from i + 1 to length(arr) - 1.
total_sum = total_sum + arr[i] + arr[j]

The question asks to optimize the time complexity. The current complexity is O(n^2). We need to achieve O(n) or O(n log n).

Let's consider the sum of all elements S.
When we pick a pair (arr[i], arr[j]), both arr[i] and arr[j] contribute to the sum.

Consider the total sum of all elements S = sum(arr).
When we form pairs, each element will be paired with every other element exactly once.

Let's take an example: arr = [1, 2, 3]
Pairs: (1, 2), (1, 3), (2, 3)
Sum of pairs = 1 + 2 + 1 + 3 + 2 + 3 = 3 + 4 + 5 = 12

Total sum S = 1 + 2 + 3 = 6

Consider the contribution of each element:
1 appears in pairs with 2 and 3.
2 appears in pairs with 1 and 3.
3 appears in pairs with 1 and 2.

Each element appears in n - 1 pairs.

Let's try a different approach.
Calculate the sum of all elements S.
Iterate through the array. For each element arr[i], the sum of pairs it forms with the remaining elements is arr[i] * (S - arr[i]). But this counts each pair twice.

Let's think about the problem differently. We want to calculate the sum of arr[i] + arr[j] for all i < j.

Consider the total sum S = sum(arr).
S^2 = (a + b + c)^2 = a^2 + b^2 + c^2 + 2ab + 2ac + 2bc
The sum of pairs is ab + ac + bc.

This also doesn't seem straightforward.

Let's go back to iterating once.
Initialize total_sum_of_pairs to 0.
Initialize sum_so_far to 0.
Iterate through the array with index i from 0 to length(arr) - 1.
For the current element arr[i], it will form pairs with all the elements that have come before it. The sum of these pairs involving arr[i] is arr[i] * sum_so_far.
After processing arr[i], add it to sum_so_far.

Let's trace with arr = [1, 2, 3]:
i = 0, arr[0] = 1, sum_so_far = 0. total_sum_of_pairs = 0 + 1 * 0 = 0. sum_so_far = 0 + 1 = 1.
i = 1, arr[1] = 2, sum_so_far = 1. total_sum_of_pairs = 0 + 2 * 1 = 2. sum_so_far = 1 + 2 = 3.
i = 2, arr[2] = 3, sum_so_far = 3. total_sum_of_pairs = 2 + 3 * 3 = 11.

Expected sum of pairs = (1+2) + (1+3) + (2+3) = 3 + 4 + 5 = 12. This is close but not quite.

Let's adjust the logic. When we are at arr[i], it needs to be paired with all subsequent elements.

Optimized approach:
Initialize total_sum_of_pairs to 0.
Initialize sum_of_remaining to the sum of all elements in the array.
Iterate through the array with index i from 0 to length(arr) - 1.
Subtract arr[i] from sum_of_remaining. Now sum_of_remaining holds the sum of elements after arr[i].
For the current element arr[i], the sum of pairs it forms with the remaining elements is arr[i] * sum_of_remaining.
Add this to total_sum_of_pairs.

Let's trace with arr = [1, 2, 3]:
sum_of_remaining = 1 + 2 + 3 = 6.
i = 0, arr[0] = 1. sum_of_remaining = 6 - 1 = 5. total_sum_of_pairs = 0 + 1 * 5 = 5.
i = 1, arr[1] = 2. sum_of_remaining = 5 - 2 = 3. total_sum_of_pairs = 5 + 2 * 3 = 11.
i = 2, arr[2] = 3. sum_of_remaining = 3 - 3 = 0. total_sum_of_pairs = 11 + 3 * 0 = 11.

Still not 12.

Let's reconsider the contribution of each pair.
The original approach correctly sums each pair once.

Let's think about the sum differently.
Sum of all pairs = sum over all i < j of arr[i] + arr[j]
= sum over all i < j of arr[i] + sum over all i < j of arr[j]

Consider the element arr[k].
In the first part, arr[k] will appear when i = k and j > k. There are n - 1 - k such cases.
In the second part, arr[k] will appear when j = k and i < k. There are k such cases.

So, the total contribution of arr[k] to the sum of pairs is arr[k] * (n - 1 - k) + arr[k] * k = arr[k] * (n - 1).

Optimized approach:
Initialize total_sum_of_pairs to 0.
Get the length of the array n.
Iterate through the array with index i from 0 to n - 1.
total_sum_of_pairs = total_sum_of_pairs + arr[i] * (n - 1)
Return total_sum_of_pairs / 2.  Because each pair is counted twice (once for i < j and once implicitly).

Let's trace with arr = [1, 2, 3], n = 3:
i = 0, arr[0] = 1. total_sum_of_pairs = 0 + 1 * (3 - 1) = 2.
i = 1, arr[1] = 2. total_sum_of_pairs = 2 + 2 * (3 - 1) = 6.
i = 2, arr[2] = 3. total_sum_of_pairs = 6 + 3 * (3 - 1) = 12.
Return 12 / 2 = 6. Still not 12.

Let's go back to the first optimized approach that almost worked.

Initialize total_sum_of_pairs to 0.
Initialize sum_so_far to 0.
Iterate through the array with index i from 0 to length(arr) - 1.
For the current element arr[i], it will form pairs with all the elements that have come before it. The sum contributed by these pairs is arr[i] * sum_so_far.
After processing arr[i], add it to sum_so_far.

Let's trace the contribution of each pair:
arr = [a, b, c]
i = 0: sum_so_far = 0.
i = 1: Pair (a, b). Contribution to sum = a + b. Our logic gives b * a. sum_so_far = a.
i = 2: Pairs (a, c), (b, c). Contribution to sum = a + c + b + c. Our logic gives c * (a + b) = ca + cb. sum_so_far = a + b.

The logic needs to accumulate the sum of pairs directly.

Optimized approach:
Initialize total_sum_of_pairs to 0.
Iterate through the array with index i from 0 to length(arr) - 1.
Iterate through the array with index j from i + 1 to length(arr) - 1.
total_sum_of_pairs = total_sum_of_pairs + arr[i] + arr[j]

This is the original O(n^2) approach. We need to do better.

Let's consider the sum of all elements S.
When we pick two elements arr[i] and arr[j] with i < j, they contribute arr[i] + arr[j] to the sum.

Consider the total sum of all elements S.
Iterate through the array. For each element arr[i], the sum of pairs it forms with subsequent elements arr[j] (where j > i) is arr[i] * (number of subsequent elements) + (sum of subsequent elements).

Optimized approach:
Initialize total_sum_of_pairs to 0.
Initialize sum_of_remaining to the sum of all elements in the array.
Iterate through the array with index i from 0 to length(arr) - 1.
sum_of_remaining = sum_of_remaining - arr[i] // Subtract the current element to get the sum of the rest
total_sum_of_pairs = total_sum_of_pairs + arr[i] * (length(arr) - 1 - i) // Contribution of arr[i] as the first element of pairs
total_sum_of_pairs = total_sum_of_pairs + sum_of_remaining // Sum of the second elements of the pairs where arr[i] is the first

Let's trace with arr = [1, 2, 3]:
sum_of_remaining = 6.
i = 0: arr[0] = 1. sum_of_remaining = 6 - 1 = 5. total_sum_of_pairs = 0 + 1 * (3 - 1 - 0) = 2. total_sum_of_pairs = 2 + 5 = 7.
i = 1: arr[1] = 2. sum_of_remaining = 5 - 2 = 3. total_sum_of_pairs = 7 + 2 * (3 - 1 - 1) = 7 + 2 * 1 = 9. total_sum_of_pairs = 9 + 3 = 12.
i = 2: arr[2] = 3. sum_of_remaining = 3 - 3 = 0. total_sum_of_pairs = 12 + 3 * (3 - 1 - 2) = 12 + 3 * 0 = 12. total_sum_of_pairs = 12 + 0 = 12.

This seems correct. The time complexity is O(n) because we iterate through the array once.

Now, let's write the pseudocode in Rust style.

Rust

fn optimized_sum_of_pairs(arr: &[i32]) -> i32 {
    let n = arr.len();
    if n < 2 {
        return 0; // No pairs possible
    }

    let mut total_sum_of_pairs = 0;
    let mut sum_of_remaining: i32 = arr.iter().sum();

    for i in 0..n {
        sum_of_remaining -= arr[i];
        total_sum_of_pairs += arr[i] * (n as i32 - 1 - i as i32); // Contribution of arr[i] as first element
        total_sum_of_pairs += sum_of_remaining; // Sum of second elements
    }

    total_sum_of_pairs
}
Let's double-check the formula.
Consider arr = [1, 2, 3]
i = 0: sum_remaining = 6 - 1 = 5. total_sum = 0 + 1 * (2) + 5 = 7 (Pairs: 1+2, 1+3)
i = 1: sum_remaining = 5 - 2 = 3. total_sum = 7 + 2 * (1) + 3 = 12 (Pair: 2+3)
i = 2: sum_remaining = 3 - 3 = 0. total_sum = 12 + 3 * (0) + 0 = 12

The formula seems correct.

Now, let's write the pseudocode.

fn optimized_sum_of_pairs(arr: &[i32]) -> i32 {
    let n = length of arr
    if n < 2 then
        return 0
    end if

    let mut total_sum_of_pairs = 0
    let mut sum_of_remaining = sum of all elements in arr

    for i from 0 to n - 1 do
        sum_of_remaining = sum_of_remaining - arr[i]
        total_sum_of_pairs = total_sum_of_pairs + arr[i] * (n - 1 - i)
        total_sum_of_pairs = total_sum_of_pairs + sum_of_remaining
    end for

    return total_sum_of_pairs
} */

//The below code was generated using Google Gemini for learning purposes.
fn optimized_sum_of_pairs(arr: &[i32]) -> i32 {
    let n: usize = arr.len();
    if n < 2 {
        return 0;
    }

    let mut total_sum_of_pairs: i32 = 0;
    let mut sum_of_remaining: i32 = arr.iter().sum();

    for i in 0..n {
        sum_of_remaining -= arr[i];
        total_sum_of_pairs += arr[i] * (n as i32 - 1 - i as i32);
        total_sum_of_pairs += sum_of_remaining;
    }

    total_sum_of_pairs
}

/* Here is a copy of the corresponding prompt, which asked Gemini to explain what optimized_sum_of_pairs does
and why it is an optimized version of the pseudocode for procedure 1.14. 
The Rust code you provided is an optimized version of the original pseudocode for calculating the sum of all pairs in an array. Here's a breakdown of what the code does and why it's optimized:

What the Code Does:

The optimized_sum_of_pairs function takes a slice of integers arr as input and returns the sum of all unique pairs of elements within that array. It achieves this with a single pass through the array.

1. Initialization:
n = arr.len();: Gets the number of elements in the array.
if n < 2 { return 0; }: Handles the edge case where there are fewer than two elements in the array, in which case no pairs can be formed, so the sum of pairs is 0.
total_sum_of_pairs = 0;: Initializes a variable to store the total sum of all pairs.
sum_of_remaining: i32 = arr.iter().sum();: Calculates the sum of all elements in the array and stores it in sum_of_remaining. This variable will be used to efficiently calculate the sum of elements that come after the current element in the iteration.

2. Single Loop Iteration:
for i in 0..n { ... }: The code iterates through the array once, with i being the index of the current element.
sum_of_remaining -= arr[i];: In each iteration, the current element arr[i] is subtracted from sum_of_remaining. After this line, sum_of_remaining holds the sum of all elements in the array with an index greater than i.
total_sum_of_pairs += arr[i] * (n as i32 - 1 - i as i32);: This part calculates the contribution of the current element arr[i] when it's paired with all the elements that come after it. There are n - 1 - i elements after arr[i]. So, arr[i] is effectively added n - 1 - i times to the total sum (once for each pair it forms with a subsequent element).
total_sum_of_pairs += sum_of_remaining;: This line adds the sum of all the elements that come after arr[i] to total_sum_of_pairs. This accounts for the second element in each pair where arr[i] is the first element.

3.Return Value:
return total_sum_of_pairs;: Finally, the function returns the calculated total_sum_of_pairs.
Why it's an Optimized Version:

The original pseudocode had a time complexity of O(n^2) due to the nested loops. The optimized Rust code achieves a time complexity of O(n) because it iterates through the array only once.

Here's why the optimized approach is more efficient:

Avoids Redundant Calculations: The original code explicitly calculates the sum of each pair. The optimized code avoids this by cleverly using the sum_of_remaining variable.
Single Pass: By maintaining the sum of the remaining elements, the optimized code can determine the contribution of each element to the total sum of pairs in a single pass through the array.
In essence, the optimization works by recognizing a pattern in how each element contributes to the total sum of pairs. For each element arr[i], it will be paired with every subsequent element. Instead of iterating through all those subsequent elements again (as the original code does), the optimized code directly calculates the sum of these pairs using the sum of the remaining elements.
*/

// Unit tests for all Tasks, generated using Google Gemini.

#[cfg(test)]
mod tests {
    use super::*;

    // Unit tests for Task #1
    #[test]
    fn test_task1_empty_array() {
        let arr = [];
        assert_eq!(binary_search_for_first_occurrence(&arr, 5), None);
    }

    #[test]
    fn test_task1_single_element_found() {
        let arr = [5];
        assert_eq!(binary_search_for_first_occurrence(&arr, 5), Some(0));
    }

    #[test]
    fn test_task1_single_element_not_found() {
        let arr = [5];
        assert_eq!(binary_search_for_first_occurrence(&arr, 3), None);
    }

    #[test]
    fn test_task1_target_at_beginning() {
        let arr = [2, 5, 7, 8, 11, 12];
        assert_eq!(binary_search_for_first_occurrence(&arr, 2), Some(0));
    }

    #[test]
    fn test_task1_target_at_end() {
        let arr = [2, 5, 7, 8, 11, 12];
        assert_eq!(binary_search_for_first_occurrence(&arr, 12), Some(5));
    }

    #[test]
    fn test_task1_target_in_middle() {
        let arr = [2, 5, 7, 8, 11, 12];
        assert_eq!(binary_search_for_first_occurrence(&arr, 8), Some(3));
    }

    #[test]
    fn test_task1_target_not_present() {
        let arr = [2, 5, 7, 8, 11, 12];
        assert_eq!(binary_search_for_first_occurrence(&arr, 6), None);
    }

    #[test]
    fn test_task1_target_present_multiple_times_at_beginning() {
        let arr = [2, 2, 2, 5, 7, 8];
        assert_eq!(binary_search_for_first_occurrence(&arr, 2), Some(0));
    }

    #[test]
    fn test_task1_target_present_multiple_times_in_middle() {
        let arr = [2, 5, 7, 7, 7, 8, 11];
        assert_eq!(binary_search_for_first_occurrence(&arr, 7), Some(2));
    }

    #[test]
    fn test_task1_target_present_multiple_times_at_end() {
        let arr = [2, 5, 7, 8, 11, 12, 12, 12];
        assert_eq!(binary_search_for_first_occurrence(&arr, 12), Some(5));
    }

    #[test]
    fn test_task1_target_less_than_all_elements() {
        let arr = [5, 7, 9];
        assert_eq!(binary_search_for_first_occurrence(&arr, 3), None);
    }

    #[test]
    fn test_task1_target_greater_than_all_elements() {
        let arr = [1, 3, 5];
        assert_eq!(binary_search_for_first_occurrence(&arr, 7), None);
    }

    // Unit tests for Task #2
    #[test]
    fn test_task2_empty_array() {
        let arr = [];
        assert_eq!(optimized_sum_of_pairs(&arr), 0);
    }

    #[test]
    fn test_task2_single_element_array() {
        let arr = [5];
        assert_eq!(optimized_sum_of_pairs(&arr), 0);
    }

    #[test]
    fn test_task2_two_elements_array() {
        let arr = [1, 2];
        assert_eq!(optimized_sum_of_pairs(&arr), 1 + 2);
    }

    #[test]
    fn test_task2_three_elements_array() {
        let arr = [1, 2, 3];
        assert_eq!(optimized_sum_of_pairs(&arr), (1 + 2) + (1 + 3) + (2 + 3));
    }

    #[test]
    fn test_task2_array_with_duplicates() {
        let arr = [1, 2, 2, 3];
        assert_eq!(
            optimized_sum_of_pairs(&arr),
            (1 + 2) + (1 + 2) + (1 + 3) + (2 + 2) + (2 + 3) + (2 + 3)
        );
    }

    #[test]
    fn test_task2_array_with_negative_numbers() {
        let arr = [-1, 2, -3];
        assert_eq!(
            optimized_sum_of_pairs(&arr),
            (-1 + 2) + (-1 + -3) + (2 + -3)
        );
    }

    #[test]
    fn test_task2_larger_array() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(
            optimized_sum_of_pairs(&arr),
            (1 + 2) + (1 + 3) + (1 + 4) + (1 + 5) + (2 + 3) + (2 + 4) + (2 + 5) + (3 + 4) + (3 + 5) + (4 + 5)
        );
    }

    #[test]
    fn test_task2_array_with_zeros() {
        let arr = [0, 1, 0, 2];
        assert_eq!(
            optimized_sum_of_pairs(&arr),
            (0 + 1) + (0 + 0) + (0 + 2) + (1 + 0) + (1 + 2) + (0 + 2)
        );
    }
}