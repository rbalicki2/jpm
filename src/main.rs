use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example!")]

struct Opt {
    #[structopt(short, long)]
    foo: bool,
}

fn main() {
    let opt = Opt::from_args();
    println!("Hello, world! {:?}", opt);
}
