struct Extrema<'s, T: 's> {
    least: &'s T,
    greatest: &'s T,
}

fn find_extrema<T: PartialOrd>(slice: &[T]) -> Extrema<T> {
    let mut least = &slice[0];
    let mut greatest = &slice[0];

    for item in &slice[1..] {
        if item < least {
            least = item;
        }
        if item > greatest {
            greatest = item;
        }
    }
    Extrema { least, greatest }
}

fn main() {}
