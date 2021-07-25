use std::fs;
use std::path;

const STAR_PATH: &str = "./star.txt";

pub struct Star {
  pub name: String,
}

pub fn parse_stars() -> Vec<Star> {
  let star_file = path::Path::new(STAR_PATH);
  if let Ok(content) = fs::read_to_string(star_file) {
    content
      .split("\n")
      .map(|s| Star {
        name: String::from(s),
      })
      .collect::<Vec<Star>>()
  } else {
    fs::write(star_file, "").expect("star file create error");
    vec![]
  }
}

pub fn is_star(stars: &Vec<Star>, title: &str) -> bool {
  if let Some(_) = stars.iter().find(|star| star.name == title) {
    true
  } else {
    false
  }
}

pub fn add(name: &str) {
  let star_file = path::Path::new(STAR_PATH);
  if let Ok(mut content) = fs::read_to_string(star_file) {
    content.push_str(&name);
    content.push_str("\n");
    fs::write(star_file, content).expect("add star error");
  }
}

pub fn remove(name: &str) {
  let star_file = path::Path::new(STAR_PATH);
  let mut stars = parse_stars();
  if let Some(index) = stars.iter().position(|s| s.name == name) {
    stars.remove(index);
    let content = stars.iter().map(|s| String::from(&s.name)).collect::<Vec<String>>().join("\n");
    fs::write(star_file, content).expect("remove star error");
  }
}

pub fn list() {
  let stars = parse_stars();
  for s in stars.iter() {
    println!("{}", s.name);
  }
}

pub fn clean() {
  let star_file = path::Path::new(STAR_PATH);
  fs::write(star_file, "").expect("clean star error");
}
