func add{range_check_ptr}(a: felt, b: felt) -> (res: felt) {
    let res = a + b;
    return (res,);
}

func sub{range_check_ptr}(a: felt, b: felt) -> (res: felt) {
    let res = a - b;
    return (res,);
}