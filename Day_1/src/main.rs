fn main() {
    //println!("{:?}",parse_calibration_val_by_char(get_file_string("test.txt")));
    println!("{:?}",locate_calibration_value(get_file_string("test.txt")));
}

use std::borrow::{Borrow, BorrowMut};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead,BufReader,Error};
use std::{string, usize};

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

fn init_int_builder (building_int:&mut HashMap<u8,Vec<char>>) -> Vec<&str>{
    let string_list:Vec<&str>  = vec![
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        
    ];
    for (index, each_string) in string_list.clone().into_iter().enumerate()
    {
        building_int.insert(1, each_string.chars().collect()); //one
    }
    string_list
}

fn init_uniue_letters (building_int:&Vec<&str>) -> HashMap<u8, HashSet<char>> {
    // Hashmap<index, HashSet<char>>
    let mut unique_letters: HashMap<u8, HashSet<char>> = HashMap::new();
    let mut max_word_length:u8 = 0;


    for word in building_int
    {
        if word.len() as u8 > max_word_length{
            max_word_length = word.len() as u8;
        }
    }
    // dbg!(max_word_length);

    for ii in 0..max_word_length
    {
        let new_hash:HashSet<char> = HashSet::new();
        unique_letters.insert(ii, new_hash.clone());
    }

    for unique_word_chars in building_int
    {
        // dbg!(String::from(*unique_word_chars));
        for (position, letter) in unique_word_chars.chars().enumerate()
        {
            //first check if there is a hashset for this position [{"O"},"N","E"] => index 0
            let mut new_hash:HashSet<&char> =  HashSet::new();
            let mut current_set = unique_letters.entry(position as u8).or_insert_with(|| HashSet::new());

            current_set.insert(letter);

        }
    }
    // dbg!(&unique_letters);
    return unique_letters
}


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
fn locate_calibration_value(raw_sypher:Vec<String>) -> i32{

    let mut keys_hash: HashMap<u8, u32> = HashMap::new();
    
    keys_hash.insert(1, 0);
    keys_hash.insert(2, 0);
    
    let mut building_int: HashMap<u8, Vec<char>> = HashMap::new().to_owned();
    let stringList: Vec<&str> = init_int_builder(&mut building_int);

    let unique_letters = init_uniue_letters(&stringList);
    
    
    let mut sums: Vec<i32> = Vec::new();
    let mut total: i32 = 0;

    const INDEX_MIN:u8 = 48;  //  0
    const INDEX_MAX:u8 = 57;  //  9

    let  mut s_att_word_counter: usize = 0; 
    let mut attempted_word: Vec<char> = vec![];


    for (index, each_line) in raw_sypher.iter().enumerate(){
        for each_char in each_line.chars(){
            match each_char as u8
            {
                
                b'a'..=b'z' => 
                {    
                    //attempt word
                        // this is based on static count which tracks which set to be referencing\
                    let temp_allocation:HashSet<char> = HashSet::new(); //%%% Need to figure out a better method
                    let current_subset: &HashSet<char> = unique_letters.get(&(s_att_word_counter as u8)).unwrap_or(&temp_allocation);
                    if current_subset.contains(&each_char){
                        s_att_word_counter+=1;
                        attempted_word.push(each_char);
                        
                    }else {
                        attempted_word.clear();
                        s_att_word_counter = 0;
                    }
                    let attempted_word_check:String = attempted_word.iter().collect();
                        // dbg!(s_att_word_counter);
                        // dbg!(&attempted_word_check);
                        if stringList.contains(&attempted_word_check.as_str())
                        {
                            let new_val_1 = word_to_int(&attempted_word_check);
                            keys_hash.insert(1,new_val_1);
                            keys_hash.insert(2,new_val_1);
                            print!("[{}] \t{} {}:", index, attempted_word_check,new_val_1);
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
                            break;
                        }
                    //todo!()
                },
                INDEX_MIN..=INDEX_MAX=> {
                    //dbg!(each_char);
                    let new_val_1 = (each_char as u8 - INDEX_MIN)as u32;
                    // dbg!((each_char, new_val_1));
                    keys_hash.insert(1,new_val_1);
                    keys_hash.insert(2,new_val_1);
                    print!("[{}] \t {}:", index, new_val_1);
                    break;
                },    
                _ => {
                    
                }
            }
        }
        println!("{:?}\n",each_line);
        
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
                
                b'a'..=b'z' => 
                {    
                    //attempt word
                        // this is based on static count which tracks which set to be referencing\
                    let temp_allocation:HashSet<char> = HashSet::new(); //%%% Need to figure out a better method
                    let current_subset: &HashSet<char> = unique_letters.get(&(s_att_word_counter as u8)).unwrap_or(&temp_allocation);
                    if current_subset.contains(&each_char){
                        s_att_word_counter+=1;
                        attempted_word.push(each_char);
                        
                    }else {
                        attempted_word.clear();
                        s_att_word_counter = 0;
                    }
                    let attempted_word_check:String = attempted_word.iter().collect();
                        // dbg!(s_att_word_counter);
                        // dbg!(&attempted_word_check);
                        // dbg!(&attempted_word_check);
                    if stringList.contains(&attempted_word_check.as_str())
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
                    //todo!()
                },    
                INDEX_MIN..=INDEX_MAX=> {
                    //dbg!(each_char);
                    let new_val_2 = (each_char as u8 - INDEX_MIN)as u32;
                    //dbg!(new_val_2);
                    keys_hash.insert(2,new_val_2);
                    
                    
                },
                _ => {
                    
                }
            }
        }
        let first = keys_hash.get(&1).cloned().unwrap_or_default();
        let mut second = keys_hash.get(&2).cloned().unwrap_or_default();
        if first == 0 {
            println!("No number found")
        }
        else {
            // dbg!((10*first + second));
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
fn word_to_int(validated_string :&String) -> u32
{
    match validated_string.as_str(){
        "zero" => 0,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => {
            println!("[CATASTROPHIC ERROR]");
            0
        }
    }
}