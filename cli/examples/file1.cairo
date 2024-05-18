%builtins output range_check

func add{range_check_ptr}(a: felt, b: felt) -> (res: felt) {
    let res = a + b;
    return (res,);
}

func main{output_ptr: felt*, range_check_ptr}() {
    alloc_locals;
    let (local sum) = add(2, 2);
    assert sum = 4;
    return ();
}