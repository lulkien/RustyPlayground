pub fn str_str(haystack: String, needle: String) -> i32 {
    haystack.find(&needle).map_or(-1, |index| index as i32)
}

fn main() {

}
