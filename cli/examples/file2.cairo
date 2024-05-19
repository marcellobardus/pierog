%builtins output range_check

from dep.file2_dep import add

func main{output_ptr: felt*, range_check_ptr}() {
    alloc_locals;
    let (local sum) = add(2, 2);
    assert sum = 4;
    return ();
}
