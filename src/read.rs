use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn read_file(path: &str) -> (Vec<(usize,usize)>,usize) {
    //the input data is all youtube links so I want to convert unique youtube links to integers
    let mut output: Vec<(usize,usize)> = Vec::new();
    let file = File::open(path).expect("couldn't open file");
    // I'm using a HashMap to correlate a unique link to an interger and a HashSet to determine if the link is unique in the first place
    let mut link_to_int: HashMap<String, usize> = HashMap::new();
    let mut unique_links: HashSet<String> = HashSet::new();
    let buf_reader = std::io::BufReader::new(file).lines();
    for line in buf_reader {
        let line_str = line.expect("couldn't read line");
        let collected_links: Vec<&str> = line_str.trim().split(",").collect(); //splitting each line up by , cuz csv
        let starting_node = collected_links[0].to_string();
        let ending_node = collected_links[1].to_string(); //making them Strings cuz can't be references for insert()
        if !unique_links.contains(&starting_node){ // passing reference so that memory is not moved
            unique_links.insert(starting_node.clone()); // have to clone so not consumed
            link_to_int.insert(starting_node.clone(),unique_links.len()-1 as usize); //using the length of the HashSet as a shortcut for getting a unique integer
        }
        if !unique_links.contains(&ending_node){ // passing reference so that memory is not moved
            unique_links.insert(ending_node.clone()); // have to clone so not consumed
            link_to_int.insert(ending_node.clone(),unique_links.len()-1 as usize); //using the length of the HashSet as a shortcut for getting a unique integer
        output.push((link_to_int[&starting_node],link_to_int[&ending_node]));
        } // just doing it again for the ending node, it's not worth making a function for cuz doing it only twice
    }
    return (output, unique_links.len())
}