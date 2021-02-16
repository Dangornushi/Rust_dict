use std::fs::File;
use std::io::{BufRead, BufReader};
use std::convert::TryInto;
use std::collections::HashMap;

fn split( word: String, n1: usize, n2: usize ) -> String {
    let str1 = word.chars().enumerate().filter(|&(i,_)| i >= n1 && i < n2).fold("".to_string(), |s, (_, c)| format!("{}{}", s, c));
    return str1;
}

fn strlen(word: &String) -> usize{
    let test = word.len().try_into().unwrap();
    return test;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut map = HashMap::new();
    for result in BufReader::new(File::open("ans.txt")?).lines() {
        let data = result?;
        let _data = &data;
        let sdata: usize = strlen( &data );
        let spdata: String = split(_data.to_string(), sdata-1, sdata);
        if spdata == ":" {
            for result2 in BufReader::new(File::open("ans.txt")?).lines(){
                let fdata = result2?;
                let _fdata = &fdata;
                //let sfdata: usize = strlen( &fdata );
                let ret = split(_fdata.to_string(), 0, 8);
                map.entry(_data.to_string()).or_insert(fdata);
                if ret == "    ret " {
                    break;
                }
            }
        } else { 
            println!("{}", map["main():"]);
        }
    }
    Ok(())
}
