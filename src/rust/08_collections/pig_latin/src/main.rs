use std::io;

const CONSONANTS: [char; 40] = ['b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't',
    'v', 'w', 'x', 'z',
    'B', 'C', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T',
    'V', 'W', 'X', 'Z',
];

fn main() {
    println!("Write some sentence in English:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut output = String::new();
    for word in input.split_whitespace() {
        let mut chars = word.chars();
        let (first, rest) = (chars.next().unwrap(), chars.as_str());
        if CONSONANTS.contains(&first) {
            let appendix = format!("{}-{}ay ", rest, first);
            output.push_str(appendix.as_str());
        }
        else {
            output.push_str(format!("{}-hay ", word).as_str());
        }
    }
    println!("{}", output);
}
