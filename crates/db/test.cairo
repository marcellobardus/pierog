// Builtin is "pre-defined" optimized execution unit.
// Using builtins, we can perform expensive operation that is not doable in Cairo CPU
// Basically memory first allocate continuous area of defined builtin, and CPU can just access it.
%builtins output

from starkware.cairo.common.alloc import alloc
from starkware.cairo.common.serialize import serialize_word

// Cairo always have to pass size if the array is passed as an argument.
// arr points to the array of size elements.
func array_sum(arr: felt*, size) -> felt {
    if (size == 0) {
        return 0;
    }

    // Cairo memory is immutable, so we can't modify the value inside the loop.
    // Meaning, we cannot just sum all the elements in the array in a loop while update value of `sum` value.
    // So we use recursion so that it can spawn new memory cells for each iteration.
    let rest_sum = array_sum(arr=arr + 1, size=size - 1);
    return arr[0] + rest_sum;
}

// Main function.
// This is the starting point of the program.

// output_ptr is a pointer to the output area of the program.
// This implicit argument is passed due to we invoke the builtins output.
func main{output_ptr: felt*}() {
    const ARRAY_SIZE = 4;

    // Allocate an array.
    let (ptr) = alloc();

    // Assert will perform assignment if value is not assigned to a variable.
    // Then it perform assertion on the value.
    assert [ptr] = 1;
    assert [ptr + 1] = 2;
    assert [ptr + 2] = 3;
    assert [ptr + 3] = 4;
    assert [ptr + 1] = 2;

    // Call array_sum to compute the sum of the elements.
    let sum = array_sum(arr=ptr, size=ARRAY_SIZE);

    // Write the sum to the program output.
    serialize_word(sum);

    return ();
}
