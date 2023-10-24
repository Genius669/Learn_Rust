// rust中常量命名全部为大写
// 函数内也可以定义，生命周期不同
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    mutability();
    const_variable();
    shadow_variable();
    shadow_variable_let();
}

fn mutability() {
    // let x = 5;// 不能对不可变变量二次赋值
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn const_variable() {
    println!("The value of const is: {}", THREE_HOURS_IN_SECONDS);
}

fn shadow_variable() {
    let x = 5;
    let x = x + 1; // 再次使用let 可以给同一个变量赋值。赋值完仍然不可变
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);
}
fn shadow_variable_let() {
    let spaces = "   ";
    let spaces = spaces.len(); // 不仅可以改变值，还可以改变类型
                               // mut可以改变值，但是不能改变类型
    println!("The length of spaces is: {}", spaces);
}
