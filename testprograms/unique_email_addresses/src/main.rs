use std::collections::HashMap;
use std::vec::Vec;

fn max_unique_valid_email(emails: Vec::<&str>) -> String {

    // rules for emails:
    // - a valid email is one that has a local and domain name separated by an @
    // - periods are removed
    // - pluses filter out everything after
    // - neither of these apply to domain names. just the local name
    // 
    // e.g. benjaminye.email+asdasdasdasd@gmail.com -> benjaminyeemail@gmail.com
    //      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ^^^^^^^^^
    //                  local              domain


    let mut map = HashMap::new();

    // clean all the emails and get counts
    for email in emails {
        
        // skip if not valid email (not exactly one @)
        let substrings: Vec::<&str> = email.split('@').collect();
        if substrings.len() != 2 {
            continue;
        }
        
        // clean local name (remove ., ignore after +)
        let mut local_name: &str = substrings[0];
        if let Some(index) = local_name.find('+') {
            local_name = &local_name[..index]
        }
        let local_name = &local_name.replace(".", "");
        let cleaned_email = [local_name, substrings[1]].join("@");

        // add to count in hashmap
        *map.entry(cleaned_email).or_insert(0) += 1;
    }

    // TODO: get highest count cleaned email
    let mut max_string: &str = "";
    let mut max_val: i32 = 0;
    for (key, val) in map.iter() {
        if *val > max_val {
            max_val = *val;
            max_string = key;
        }
    }

    max_string.into()
}

fn main() {
    let emails1 = vec![
        "test.email+alex@u.northwestern.edu",
        "test.e.mail+bob.cathy@u.northwestern.edu",
        "testemail+david@u.north.western.edu",
        "thisisnotanemail",
    ];

    println!("{}", max_unique_valid_email(emails1));
    
    let emails2 = vec![
        "aa@gmail.com",
        "aa+123123123123@gmail.com",
        "a.a+iiiiii@gmail.com",
        "b@gmail.com",
        "b@gmail.com",
        "bbbbb@gmail.com",
        "c@gmail.com",
        "c@gmail.com",
        "word",
        "word",
        "word",
        "word",
        "word",
        "word",
    ];

    println!("{}", max_unique_valid_email(emails2));
}
