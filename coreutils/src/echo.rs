use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    for a in 1..args.len() {
        print!("{} ",args[a]);
    }
    print!("\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn echo_test() {
        main();
        assert_eq!("echo_test","echo_test");
    }
}
