use function_timer::time;

struct Test {}

#[time(disable)]
impl Test {
    fn test() {}
}
