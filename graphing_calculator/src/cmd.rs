pub fn run(
  args: Vec<String>,
  commands: Vec<String>,
  functions: Vec<&dyn Fn(Vec<String>) -> ()>,
) -> () {
  let mut args = args;
  args.remove(0);
  if args.len() == 0 {
    println!(
      "Usage:
    draw <in file (.graph)> <out file (.svg)> <title>
    solve <equation>
    simultaneous <equation 1> <equation 2>"
    );
    return;
  } else {
    match commands.iter().position(|x| *x == args[0]) {
      Some(i) => {
        args.remove(0);
        functions[i](args)
      }
      None => println!("Command not found."),
    }
  }
}
