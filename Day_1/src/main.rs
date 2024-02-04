fn main() {
    let file_name: &str = "mini_test.txt";
    // let file_name: &str = "test.txt";
    let raw_sypher = get_file_string(file_name);
    // println!("{:?}",locate_calibration_value(get_file_string("mini_test.txt")));
    let string_list:Vec<&str>  = vec![
        /* "zero",*/       "one",        "two",        "three",        "four",
        "five",        "six",        "seven",        "eight",        "nine",
        
    ];
    let mut building_int: HashMap<u8, Vec<char>> = HashMap::new();

    let mut unique_letters: HashMap<u8, HashSet<char>> = HashMap::new(); 
    let mut rev_unique_letters: HashMap<u8, HashSet<char>> = HashMap::new();  //how to use box?


    let mut keys_hash: HashMap<u8, u32> = HashMap::new();
    
    keys_hash.insert(1, 0);
    keys_hash.insert(2, 0);



    let binding = reverse_string_list(string_list.clone());
    let rev_string_list: Vec<&str> = binding.iter().map(|s| s.as_str()).collect();



    init_int_builder(&string_list,&mut building_int);
    init_int_builder(&rev_string_list,&mut building_int);


    saturate_unique_word_hashset(&string_list,&mut unique_letters);
    saturate_unique_word_hashset(&rev_string_list,&mut rev_unique_letters);


    // dbg!(unique_letters);
    // dbg!(rev_unique_letters);

    let firsts = find_firsts(string_list,&raw_sypher,&mut keys_hash,&mut unique_letters);
    let lasts = find_lasts(rev_string_list,&raw_sypher,&mut keys_hash,&mut rev_unique_letters);
    dbg!(firsts+lasts);
    //find_lasts();
}

fn find_firsts(string_list:Vec<&str>, raw_sypher:&Vec<String> , keys_hash:&mut HashMap<u8, u32>, unique_letters: &mut HashMap<u8, HashSet<char>>) -> u32{
    //parse the string forwards,
    //stop every time you find the first
    //multiply the result of each number by 10
    //sum total

    let mut total: u32 = 0;
    
    const INDEX_MIN:u8 = 48;  //  0
    const INDEX_MAX:u8 = 57;  //  9
    
    let  mut s_att_word_counter: usize = 0; 
    let mut attempted_word: Vec<char> = vec![];


    for (index, each_line) in raw_sypher.iter().enumerate(){
        for (sub_dex, each_char) in each_line.chars().enumerate(){
            match each_char as u8
            {
                INDEX_MIN..=INDEX_MAX=> {
                    //dbg!(each_char);
                    let new_val_1 = (each_char as u8 - INDEX_MIN)as u32;
                    // dbg!((each_char, new_val_1));
                    keys_hash.insert(1,new_val_1);
                    // print!("[{}] \t {}:", index, new_val_1);
                    s_att_word_counter = 0;
                    attempted_word.clear();
                    break;
                },
                b'a'..=b'z' => 
                {    
                    //attempt word
                    // this is based on static count which tracks which set to be referencing\
                    let temp_allocation:HashSet<char> = HashSet::new(); //%%% Need to figure out a better method
                    let current_subset: &HashSet<char> = unique_letters.get(&(s_att_word_counter as u8)).unwrap_or(&temp_allocation);
                    if current_subset.contains(&each_char){
                        s_att_word_counter+=1;
                        attempted_word.push(each_char);
                        // dbg!(each_char);
                        // println!("newchar added: {}", each_char);
                    }
                    let attempted_word_check:String = attempted_word.iter().collect();
                    // dbg!(s_att_word_counter);
                    // dbg!(&attempted_word_check);
                    if string_list.contains(&attempted_word_check.as_str())
                    {
                            let new_val_1 = word_to_int(&attempted_word_check);
                            keys_hash.insert(1,new_val_1);
                            // print!("[{}] \t{} {}:", index, attempted_word_check,new_val_1);
                            // if let Ok(ii) = attempted_word_check.clone().trim().parse::<u32>(){
                            //     dbg!(ii);
                            //     // println!("{}",ii)
                            // }
                            // else {
                            //     dbg!(&attempted_word_check);
                            // }
                            // // println!("new word: {:?} at [{}]",attempted_word_check,s_att_word_counter);
                            // dbg!(&attempted_word_check);
                            // // let test: i32= attempted_word_check.trim().parse().expect("Que?");
                            // // dbg!(test);
                            // //keys_hash.insert(1,test);
                            attempted_word.clear();
                            s_att_word_counter = 0;
                            break;
                    }
                    if !current_subset.contains(&each_char) {
                        attempted_word.clear();
                        s_att_word_counter = 0;
                    }
                },
                
                _ => {
                    //not alphanumeric
                }
            }
        }
        
        let first = keys_hash.get(&1).cloned().unwrap_or_default();
        
        total += first*10;
        println!("\"FIRSTS: \"[{:4.}] {:1.} ={} #[{:?}]",index+1,first*10 ,total,each_line);
    }
    total

}

fn find_lasts(rev_string_list:Vec<&str>, raw_sypher:&Vec<String> , keys_hash:&mut HashMap<u8, u32>, rev_unique_letters: &mut HashMap<u8, HashSet<char>>) -> u32{
    //parse the string forwards,
    //stop every time you find the first
    //multiply the result of each number by 10
    //sum total

    let mut total: u32 = 0;
    
    const INDEX_MIN:u8 = 48;  //  0
    const INDEX_MAX:u8 = 57;  //  9
    
    let  mut s_att_word_counter: usize = 0; 
    let mut attempted_word: Vec<char> = vec![];


    for (index, each_line) in raw_sypher.iter().enumerate(){
        for (sub_dex, each_char) in each_line.chars().rev().enumerate(){
            match each_char as u8
            {
                INDEX_MIN..=INDEX_MAX=> {
                    //dbg!(each_char);
                    let new_val_2 = (each_char as u8 - INDEX_MIN)as u32;
                    // dbg!((each_char, new_val_1));
                    keys_hash.insert(2,new_val_2);
                    // print!("[{}] \t {}:", index, new_val_1);
                    s_att_word_counter = 0;
                    attempted_word.clear();
                    break;
                },
                b'a'..=b'z' => 
                {    
                    //attempt word
                    // this is based on static count which tracks which set to be referencing\
                    let temp_allocation:HashSet<char> = HashSet::new(); //%%% Need to figure out a better method
                    let current_subset: &HashSet<char> = rev_unique_letters.get(&(s_att_word_counter as u8)).unwrap_or(&temp_allocation);
                    if current_subset.contains(&each_char){
                        s_att_word_counter+=1;
                        attempted_word.push(each_char);
                        // dbg!(each_char);
                        // println!("newchar added: {}", each_char);
                    }
                    let attempted_word_check:String = attempted_word.iter().collect();
                    // dbg!(s_att_word_counter);
                    // dbg!(&attempted_word_check);
                    if rev_string_list.contains(&attempted_word_check.as_str())
                    {
                            let new_val_2 = word_to_int(&attempted_word_check);
                            keys_hash.insert(2,new_val_2);
                            // print!("[{}] \t{} {}:", index, attempted_word_check,new_val_1);
                            // if let Ok(ii) = attempted_word_check.clone().trim().parse::<u32>(){
                            //     dbg!(ii);
                            //     // println!("{}",ii)
                            // }
                            // else {
                            // dbg!(&attempted_word_check);
                            // }
                            // // println!("new word: {:?} at [{}]",attempted_word_check,s_att_word_counter);
                            // dbg!(&attempted_word_check);
                            // // let test: i32= attempted_word_check.trim().parse().expect("Que?");
                            // // dbg!(test);
                            // //keys_hash.insert(1,test);
                            attempted_word.clear();
                            s_att_word_counter = 0;
                            break;
                    }
                    if !current_subset.contains(&each_char) {
                        attempted_word.clear();
                        s_att_word_counter = 0;
                    }
                },
                
                _ => {
                    //not alphanumeric
                }
            }
        }
        
        let last = keys_hash.get(&2).cloned().unwrap_or_default();
        
        total += last;
        println!("\"LASTS: \"[{:4.}] {:1.} ={} #[{:?}]",index+1, last ,total,each_line);
    }
    total

}

use core::str;
use std::borrow::{Borrow, BorrowMut};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead,BufReader,Error};
use std::{string, usize, vec};

fn reverse_string_list(string_list:Vec<&str>) -> Vec<String>{
    string_list
        .iter()
        .map(|&original_str| original_str.chars().rev().collect())
        .collect()
}

fn get_file_string(file_name: &str) -> Vec<String>{
    
    let mut file = File::open(file_name);
    let mut file_contents: Vec<String> = Vec::new();     //String::new();
    //dbg!(&file_name);
    
    match &mut file {
        Ok(file) => {
                let mut bfr = BufReader::new(file);
                let mut line_count = 0;
                for line in bfr.lines(){
                    line_count +=1;
                    match &line
                    {
                        Ok(new_line) => {
                            file_contents.push(new_line.to_string());
                            //file byte count aquired :)
                        },
                        Err(E) => {
                            let mut err_msg_read = String::from("[USAGE] Reads opened file line by line");
                            err_msg_read += &format!("[ERROR] Error while reading file on line: [{}]",line_count); 
                            println!("{}",err_msg_read);
                        }
                    }
                }
            },
            _ =>{
                let mut err_msg_open = String::from("[USAGE] Opens a file for rust app");
                err_msg_open += &format!("\n[ERROR] File name/path not found in root directory: {}",file_name);
                println!("{}",err_msg_open);
            }
        };
        file_contents
}

fn init_int_builder (string_list:&Vec<&str>,building_int:&mut HashMap<u8,Vec<char>>){
    
    
    for (index, each_string) in string_list.clone().into_iter().enumerate()
    {
        
        building_int.insert(index as u8, each_string.chars().collect()); //one
    }
}
// fn rev_init_int_builder (string_list:&Vec<&str>,rev_building_int:&mut HashMap<u8,Vec<char>>){
   
//     for (index , each_string) in string_list.clone().into_iter().enumerate().rev()
//     {
        
//         rev_building_int.insert(index as u8, each_string.chars().rev().collect()); //one
//     }
// }

fn initialise_map_set_sub_series(len:u8) -> HashMap<u8, HashSet<char>>{
    let mut unique_letters: HashMap<u8, HashSet<char>> = HashMap::new();
    for ii in 0..len
    {
        let new_hash:HashSet<char> = HashSet::new();
        unique_letters.insert(ii, new_hash.clone());
    }
    unique_letters
}

fn saturate_unique_word_hashset(building_int:&Vec<&str>, unique_letters: &mut HashMap<u8, HashSet<char>>){
    for word_chars in building_int
    {
        // dbg!(String::from(*unique_word_chars));
        for (position, letter) in word_chars.chars().enumerate()
        {
            //first check if there is a hashset for this position [{"O"},"N","E"] => index 0
            let mut new_hash:HashSet<&char> =  HashSet::new();
            unique_letters.entry(position as u8).or_insert_with(|| HashSet::new()).insert(letter);
        }
    }
}

// fn rev_saturate_unique_word_hashset(building_int:&Vec<&str>, unique_letters: &mut HashMap<u8, HashSet<char>>){
//     for word_chars in building_int
//     {
//         // dbg!(String::from(*unique_word_chars));
//         for (position, letter) in word_chars.chars().rev().enumerate()
//         {
//             //first check if there is a hashset for this position [{"O"},"N","E"] => index 0
//             let mut new_hash:HashSet<&char> =  HashSet::new();
//             unique_letters.entry(position as u8).or_insert_with(|| HashSet::new()).insert(letter);
//         }
//     }
// }



fn init_unique_letters (building_int:&Vec<&str>) -> (HashMap<u8, HashSet<char>>,HashMap<u8, HashSet<char>>) {
    // Hashmap<index, HashSet<char>>
    //  Assumption is that it'll fail long before it gets here if somethign is wrong with the text.
    let max_word_length:u8 = get_max_word_length(building_int);

    let mut unique_letters: HashMap<u8, HashSet<char>> = initialise_map_set_sub_series(max_word_length); 
    let mut rev_unique_letters: HashMap<u8, HashSet<char>> = initialise_map_set_sub_series(max_word_length); //how to use box?
    // dbg!(max_word_length);

    
    saturate_unique_word_hashset(&building_int, &mut unique_letters);


    // dbg!(&unique_letters);
    return (unique_letters, rev_unique_letters)
}
fn get_max_word_length(building_int:&Vec<&str>) -> u8{ 
    let mut max_word_length = 0;
    for word in building_int
    {
        if word.len() as u8 > max_word_length{
            max_word_length = word.len() as u8;
        }
    }
    max_word_length
}


/*
fn parse_calibration_val_by_char(raw_sypher:Vec<String>) -> i32{
    let mut keys_hash: HashMap<u8, u32> = HashMap::new();
    
    keys_hash.insert(1, 0);
    keys_hash.insert(2, 0);
    static key_counter:i32 = 0;

    let mut building_int: HashMap<u8, Vec<char>> = HashMap::new();
    
    let stringList:Vec<&str> = init_int_builder(&mut building_int);

    let mut sums: Vec<i32> = Vec::new();
    let mut total: i32 = 0;

    const INDEX_MIN:u8 = 48;  //  0
    const INDEX_MAX:u8 = 57;  //  9

    for each_line in raw_sypher{
        for each_char in each_line.chars(){
            // dbg!((each_char,INDEX_MIN,INDEX_MAX));
            match each_char as u8
            {
                INDEX_MIN..=INDEX_MAX=> {
                    //dbg!(each_char);
                    let new_val_1 = (each_char as u8 - INDEX_MIN)as u32;
                    //dbg!(new_val_1);
                    keys_hash.insert(1,new_val_1);
                    break;
                },
                b'a'..=b'z' => 
                {    
                    //todo!()
                },    
                _ => {
                    
                }
            }
            
        }
        for each_char in each_line.chars().rev(){
            match each_char as u8
            {
                INDEX_MIN..=INDEX_MAX=> {
                    let new_val_2 = (each_char as u8 - INDEX_MIN)as u32;
                    //dbg!(new_val_2);
                    keys_hash.insert(2,new_val_2);
                    break;
                },
                _ => {
                    // Not a number println!("Horrible error when parsing chars...");
                }
            }
            
        }
        let first = keys_hash.get(&1).cloned().unwrap_or_default();
        let mut second = keys_hash.get(&2).cloned().unwrap_or_default();
        if first == 0 {
            println!("No number found")
        }
        else {
            let addition = (10*first + second)as i32;
            // dbg!(first, second, addition);
            sums.push(addition);
        }
    }

    for sum in sums
    {
        total += sum;
    }
    total
}
*/

fn locate_calibration_value(string_list:Vec<&str>, raw_sypher:Vec<String>) -> i32{

    let mut keys_hash: HashMap<u8, u32> = HashMap::new();
    
    keys_hash.insert(1, 0);
    keys_hash.insert(2, 0);
    
    let mut building_int: HashMap<u8, Vec<char>> = HashMap::new().to_owned();
    
    init_int_builder(&string_list,&mut building_int);

    let (unique_letters,_) = init_unique_letters(&string_list);
    
    
    let mut sums: Vec<i32> = Vec::new();
    let mut total: i32 = 0;
    
    const INDEX_MIN:u8 = 48;  //  0
    const INDEX_MAX:u8 = 57;  //  9
    
    let  mut s_att_word_counter: usize = 0; 
    let mut attempted_word: Vec<char> = vec![];
    

    for (index, each_line) in raw_sypher.iter().enumerate(){
        
        attempted_word.clear();
        s_att_word_counter = 0;

        for (sub_dex, each_char) in each_line.chars().enumerate(){
            match each_char as u8
            {
                INDEX_MIN..=INDEX_MAX=> {
                    //dbg!(each_char);
                    let new_val_1 = (each_char as u8 - INDEX_MIN)as u32;
                    // dbg!((each_char, new_val_1));
                    keys_hash.insert(1,new_val_1);
                    keys_hash.insert(2,new_val_1);
                    // print!("[{}] \t {}:", index, new_val_1);
                    s_att_word_counter = 0;
                    attempted_word.clear();
                    break;
                },
                b'a'..=b'z' => 
                {    
                    //attempt word
                    // this is based on static count which tracks which set to be referencing\
                    let temp_allocation:HashSet<char> = HashSet::new(); //%%% Need to figure out a better method
                    let current_subset: &HashSet<char> = unique_letters.get(&(s_att_word_counter as u8)).unwrap_or(&temp_allocation);
                    if current_subset.contains(&each_char){
                        s_att_word_counter+=1;
                        attempted_word.push(each_char);
                        // println!("newchar added: {}", each_char);
                    }
                    let attempted_word_check:String = attempted_word.iter().collect();
                    // dbg!(s_att_word_counter);
                    // dbg!(&attempted_word_check);
                    if string_list.contains(&attempted_word_check.as_str())
                    {
                            let new_val_1 = word_to_int(&attempted_word_check);
                            keys_hash.insert(1,new_val_1);
                            keys_hash.insert(2,new_val_1);
                            // print!("[{}] \t{} {}:", index, attempted_word_check,new_val_1);
                            // if let Ok(ii) = attempted_word_check.clone().trim().parse::<u32>(){
                            //     dbg!(ii);
                            //     // println!("{}",ii)
                            // }
                            // else {
                            //     dbg!(&attempted_word_check);
                            // }
                            // // println!("new word: {:?} at [{}]",attempted_word_check,s_att_word_counter);
                            // // dbg!(&attempted_word_check);
                            // // let test: i32= attempted_word_check.trim().parse().expect("Que?");
                            // // dbg!(test);
                            // //keys_hash.insert(1,test);
                            attempted_word.clear();
                            s_att_word_counter = 0;
                            break;
                    }
                    if !current_subset.contains(&each_char) {
                        attempted_word.clear();
                        s_att_word_counter = 0;
                    }
                    //todo!()
                },
                 
                _ => {
                    
                }
            }
        }
        
        attempted_word.clear();
        s_att_word_counter = 0;
        // println!("{:?}\n",each_line);
        for each_char in each_line.chars(){
            // dbg!(each_char);
            // match each_char as u8
            // {
            //     INDEX_MIN..=INDEX_MAX=> {
            //         let new_val_2 = (each_char as u8 - INDEX_MIN)as u32;
            //         //dbg!(new_val_2);
            //         keys_hash.insert(2,new_val_2);
            //         break;
            //     },
            //     b'a'..=b'z' => 
            //     {    
            //         //todo!()
            //     }, 
            //     _ => {
            //         // Not a number println!("Horrible error when parsing chars...");
            //     }
            // }
            match each_char as u8
            {
                
                INDEX_MIN..=INDEX_MAX=> {
                    //dbg!(each_char);
                    let new_val_2 = (each_char as u8 - INDEX_MIN)as u32;
                    //dbg!(new_val_2);
                    keys_hash.insert(2,new_val_2);
                    attempted_word.clear();
                    s_att_word_counter = 0;
                    
                },
                
                b'a'..=b'z' => 
                {    
                    //attempt word
                        // this is based on static count which tracks which set to be referencing\
                    let temp_allocation:HashSet<char> = HashSet::new(); //%%% Need to figure out a better method
                    let current_subset: &HashSet<char> = unique_letters.get(&(s_att_word_counter as u8)).unwrap_or(&temp_allocation);
                    if current_subset.contains(&each_char){
                        s_att_word_counter+=1;
                        attempted_word.push(each_char);
                        
                    }
                    let attempted_word_check:String = attempted_word.iter().collect();
                        // dbg!(s_att_word_counter);
                        // dbg!(&attempted_word_check);
                        // dbg!(&attempted_word_check);
                    if string_list.contains(&attempted_word_check.as_str())
                    {
                        let new_val_2 = word_to_int(&attempted_word_check);
                        // dbg!(new_val_2);
                        keys_hash.insert(2,new_val_2);
                        // dbg!(new_val_2);
                        // if let Ok(ii) = attempted_word_check.clone().trim().parse::<u32>(){
                        //     dbg!(ii);
                        //     // println!("{}",ii)
                        // }
                        // else {
                        //     dbg!(&attempted_word_check);
                        // }
                        // // println!("new word: {:?} at [{}]",attempted_word_check,s_att_word_counter);
                        // // dbg!(&attempted_word_check);
                        // // let test: i32= attempted_word_check.trim().parse().expect("Que?");
                        // // dbg!(test);
                        // //keys_hash.insert(1,test);
                    }
                    if ! current_subset.contains(&each_char){
                        attempted_word.clear();
                        s_att_word_counter = 0;
                    }
                    //todo!()
                },    
                
                _ => {
                    
                }
            }
        }
        
        /*
            This does one final pass to see if the previous character triggered a word search
            Even if the present char is numeric it'll prioritise the word.
         */
        let attempted_word_check:String = attempted_word.iter().collect();
        // dbg!(s_att_word_counter);
        // dbg!(&attempted_word_check);
        if string_list.contains(&attempted_word_check.as_str())
        {
                let new_val_1 = word_to_int(&attempted_word_check);
                keys_hash.insert(2,new_val_1);
                // print!("[{}] \t{} {}:", index, attempted_word_check,new_val_1);
                // if let Ok(ii) = attempted_word_check.clone().trim().parse::<u32>(){
                //     dbg!(ii);
                //     // println!("{}",ii)
                // }
                // else {
                //     dbg!(&attempted_word_check);
                // }
                // // println!("new word: {:?} at [{}]",attempted_word_check,s_att_word_counter);
                // // dbg!(&attempted_word_check);
                // // let test: i32= attempted_word_check.trim().parse().expect("Que?");
                // // dbg!(test);
                // //keys_hash.insert(1,test);
            
        }
        
        let first = keys_hash.get(&1).cloned().unwrap_or_default();
        let mut second = keys_hash.get(&2).cloned().unwrap_or_default();
        if first == 0 {
            println!("No number found")
        }
        else {
            // dbg!((10*first + second));
            let addition = (10*first + second)as i32;
            println!("[{:4.}] {:1.} : {:1.} ={} #[{:?}]",index,first ,second ,addition,each_line);
            // dbg!(first, second, addition);
            sums.push(addition);
        }
        keys_hash.insert(1, 0);
        keys_hash.insert(2, 0);
    }

    for sum in sums
    {
        total += sum;
    }
    total
    

}


fn word_to_int(validated_string :&String) -> u32
{
    match validated_string.as_str(){
        "zero"|"orez" => 0,
        "one"|"eno" => 1,
        "two"|"owt" => 2,
        "three"|"eerht" => 3,
        "four"|"ruof" => 4,
        "five"|"evif" => 5,
        "six"|"xis" => 6,
        "seven"|"neves" => 7,
        "eight"|"thgie" => 8,
        "nine"|"enin" => 9,
        _ => {
            println!("[CATASTROPHIC ERROR]");
            0
        }
    }
}