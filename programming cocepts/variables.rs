fn main() {
    // CONST
    // MUST have type
    // Named IN_CAPITAL_WITH_UNDERSCORE
    // Can never be muted.
    // Needs to have explicit value. The value cannot be a result from function etc.
    const MAX_POINTS: i32 = 100_000;

    // MUT
    // Can mute it's value but not it's type.
    let mut x = 5;
    x = 6;

    // SHADOWED
    // Replaces the previous variable
    // Can mute it's type
    // Can mute it's value
    // Can switch between mutable and unmutable
    let exes = 'xxxx';
    let exes = exes.len();
    let mut exes = exes + exes.len();
    exes = exes * 2;
}
