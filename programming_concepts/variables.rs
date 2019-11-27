fn main() {
    // CONST
    // MUST have type
    // Named IN_CAPITAL_WITH_UNDERSCORE
    // Can never be muted.
    // Needs to have explicit value. The value cannot be a result from function etc.
    const MAX_POINTS: i32 = 100_000;
    println!("MAX_POINTS is {}", MAX_POINTS);

    // MUT
    // Can mute it's value but not it's type.
    let mut x = 5;
    println!("x is {}", x);
    x = 6;
    println!("x is {}", x);

    // SHADOWED
    // Replaces the previous variable
    // Can mute it's type
    // Can mute it's value
    // Can switch between mutable and unmutable
    let exes = "xxxx";
    println!("exexs is {}", exes);
    let exes = exes.len();
    println!("exexs is {}", exes);
    let mut exes = exes + exes;
    println!("exexs is {}", exes);
    exes = exes * 2;
    println!("exexs is {}", exes);
}
