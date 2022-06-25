fn main() {
  let son = "Vlad";
  println!("Nice {}", son);

  let mut i = 0;
  let result = loop {
    i += 1;
    println!("{}", i);
    if i == 10 {
      break i;
    }
  };

  println!("Result {}", result);


}