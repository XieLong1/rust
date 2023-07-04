pub mod second;

pub fn print_chars() {
  for ascii in (b'Z'..=b'a').rev() {
      let c = ascii as char;
      print!("{}", c);
  }
  println!();
}