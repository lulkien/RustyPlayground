fn longest_palindrome(s: &String) -> String {
    let str_len = s.len();
    let mut l: Vec<Vec<bool>> = vec![vec![false; str_len]; str_len];
    let mut max_len = 1;
    let mut start_pos = 0;

    for idx in 0..str_len {
        l[idx][idx] = true;
    }

    // Check palindrome string with range
    for len in 2..=str_len {
        for start in 0..=str_len - len {
            let end = start + len - 1;

            if len == 2 && s.chars().nth(start) == s.chars().nth(end) {
                l[start][end] = true;
            } else {
                l[start][end] = l[start + 1][end - 1] && s.chars().nth(start) == s.chars().nth(end);
            }

            if l[start][end] && len > max_len {
                max_len = len;
                start_pos = start;
            }
        }
    }

    let result = &s[start_pos..start_pos + max_len];
    result.to_string()
}

fn main() {
    let palindrome: String = longest_palindrome(&"borcdubqiupahpwjizjjbkncelfazeqynfbrnzuvbhjnyvrlkdyfyjjxnprfocrmisugizsgvhszooktdwhehojbrdbtgkiwkhpfjfcstwcajiuediebdhukqnroxbgtvottummbypokdljjvnthcbesoyigscekrqswdpalnjnhcnqrarxuufzzmkwizptyvjhpfidgmskuaggtpxqisdlyloznkarxofzeeyvteynluofuhbllyiszszbwnsglqjkignusarxsjvctpgiwnhdufmfpanfwxjwlmhgllzsmdpncbwnsbdtsvrjybynifywkulqnzprcxockbhrhvnwxrkvwumyieazclcviumvormynbryaziijpdinwatwqppamfiqfiojgwkvfzyxadyfjrgmtttvlgkqghgbcfhkigfojjkhskzenlpasyozcsuccdxhulcubsgomvcrbqwakrraesfifecmoztayrdjicypakrrneulfbqqxtrdelggedvloebqaztmfyfkhuonrognejxpesmwgnmnnnamvkxqvzrgzdqtvuhccryeowywmbixktnkqnwzlzfvloyqcageugmjqihyjxhssmhkfzxpzxmgpjgsduogfolnkkqizitbvvgrkczmojxnabruwwzxxqcevdwvtiigwckpxnnxxxdhxpgomncttjutrsvyrqcfwxhexhaguddkiesmfrqfjfdaqbkeqinwicphktffuwcazlmerdhraufbpcznbuipmymipxbsdhuesmcwnkdvilqbnkaglloswcpqzdggnhjdkkumfuzihilrpcvemwllicoqiugobhrwdxtoefynqmccamhdtxujfudbglmiwqklriolqfkknbmindxtljulnxhipsieyohnczddabrxzjmompbtnnxvivmoyfzfrxlufowtqzojfclmatthjtbhotdoheusnpirwicbtyrcuojsdmfcx".to_string());
    println!("Longest palindrome substring: {}", palindrome);
}
