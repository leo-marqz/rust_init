fn main()
{
    let mut result = sum(5,5);
    println!("sum: {}", result);

    let numbers = vec![1, 2, 3, 4, 5];
    result = sum_list(&numbers);
    println!("sum_list: {}", result);
}

fn sum(a:i32, b:i32)-> i32
{
    return a + b;
}

fn sum_list(list:&[i32])->i32 
{
    let mut result:i32 = 0;
    for i in list
    {
        result = result + i;
    }
    return result;
}