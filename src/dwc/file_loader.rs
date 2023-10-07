use std::fs;
use crate::dwc::complex::*;

fn extract_words(input: &String) -> Vec<String>
{
    let mut words: Vec<String> = Vec::new();
    let words_str: Vec<&str> = input.split(' ').collect();

    for i in 0..words_str.len()
    {
        words.push(words_str[i].to_string());
    }

    return words;
}

fn extract_lines(input: &String) -> Vec<String>
{
    let mut words: Vec<String> = Vec::new();

    for i in input.lines()
    {
            words.push(i.to_string());
    }

    return words;
}

pub fn load_from_txt(path: &str) -> Vec<Complex>
{
    let contents: String = fs::read_to_string(path).unwrap();
    let mut points: Vec<Complex> = Vec::new();

    let lines: Vec<String> = extract_lines(&contents);

    for line in lines
    {
        let words: Vec<String> = extract_words(&line);

        let real: f32 = words[0].parse::<f32>().unwrap();
        let img: f32 = words[1].parse::<f32>().unwrap();

        points.push(Complex { real: real, img: img });
    }

    return points;
}