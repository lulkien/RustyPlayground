fn longest_common_prefix(strs: Vec<String>) -> String {
    strs.into_iter()
        .reduce(|accumulator, current| {
            accumulator
                .chars() // Get chars array from accumulator
                .zip(current.chars()) // zip 2 interators together
                .take_while(|(a, c)| a == c) // take element when that char from accumulator and current value is the same char
                .map(|(c, _)| c) // get that char, just need 1 from acc or cur
                .collect::<_>() // collect
        })
        .unwrap()
}

fn main() {
    let strs: Vec<_> = vec!["fdog 1".to_string(), "fdog2".to_string(), "fdog_3".to_string()];
    println!("{}", longest_common_prefix(strs));
}
