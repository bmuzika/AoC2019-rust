fn main() {
    let input_range= (235741..706948).collect::<Vec<u32>>();

    let no_descending = input_range.clone()
        .into_iter()
        .filter(|n| no_digit_smaller(*n))
        .map(|n| n.to_string())
        .collect::<Vec<_>>();

    let doubled_digits = no_descending.clone()
        .into_iter()
        .filter(|s| contains_doubled_digit(s.to_string()))
        .collect::<Vec<_>>();

    let final_crit = doubled_digits.clone()
        .into_iter()
        .filter(|s| no_larger_group_on_double_match(s.to_string()))
        .collect::<Vec<_>>();

    print!("Input: {} \n No descending: {} \n Doubled digit: {} \n Final crit: {}", input_range.len(), no_descending.len() ,doubled_digits.len(), final_crit.len());
}

fn no_digit_smaller(number: u32) -> bool {
    let digits = number.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();
    let mut prev_digit = 0;
    for digit in digits {
        if digit < prev_digit
        {
            return false;
        }
        prev_digit = digit;
    }
    true
}

fn contains_doubled_digit(n: String) -> bool {
    let found = n.find("00")
        .or_else(|| n.find("11")
            .or_else(|| n.find("22")
                .or_else(|| n.find("33")
                    .or_else(|| n.find("44")
                        .or_else(|| n.find("55")
                            .or_else(|| n.find("66")
                                .or_else(|| n.find("77")
                                    .or_else(||n.find("88")
                                        .or_else(|| n.find("99"))))))))));
    match found {
        Some(_) => true,
        None => false,
    }
}

fn no_larger_group_on_double_match(n: String) -> bool {
    let found22 = n.find("22");
    let found33 = n.find("33");
    let found44 = n.find("44");
    let found55 = n.find("55");
    let found66 = n.find("66");
    let found77 = n.find("77");
    let found88 = n.find("88");
    let found99 = n.find("99");

    let found222 = n.find("222");
    let found333 = n.find("333");
    let found444 = n.find("444");
    let found555 = n.find("555");
    let found666 = n.find("666");
    let found777 = n.find("777");
    let found888 = n.find("888");
    let found999 = n.find("999");

    let mut results_vec = vec![0; 8];

    if found22 != None {
        if found222 == None {
            results_vec[0] = 1;
        }
    }

    if found33 != None {
        if found333 == None {
            results_vec[1] = 1;
        }
    }

    if found44 != None {
        if found444 == None {
            results_vec[2] = 1;
        }
    }

    if found55 != None {
        if found555 == None {
            results_vec[3] = 1;
        }
    }

    if found66 != None {
        if found666 == None {
            results_vec[4] = 1;
        }
    }

    if found77 != None {
        if found777 == None {
            results_vec[5] = 1;
        }
    }

    if found88 != None {
        if found888 == None {
            results_vec[6] = 1;
        }
    }

    if found99 != None {
        if found999 == None {
            results_vec[7] = 1;
        }
    }

    let vector_sum: u32 = results_vec.into_iter().sum();

    if vector_sum > 0 {
        return true;
    }
    false
}