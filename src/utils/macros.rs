#[macro_export]
macro_rules! day_fn {
    ($($day:literal)*) => {
        fn day_fn(day: u8, part1: bool) -> impl Fn(&[String]) -> i64 {
            match day {
                $(
                $day => if part1 {
                            paste! { [<day $day>]::solve1 }
                        } else {
                            paste! { [<day $day>]::solve2 }
                        },
                )*
                _ => unimplemented!(),
            }
        }
    };
}
