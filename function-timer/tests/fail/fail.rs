use function_timer::time;

struct Test {}

impl Test {
    #[time(struct)]
    fn test() {}
}
