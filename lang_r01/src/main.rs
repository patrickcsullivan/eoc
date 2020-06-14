mod ast;
mod interp;
mod uniquify;
mod x86;

fn main() {
    println!("Hello, world!");
}

// struct Ctx {
//     counter: u64,
// }

// impl Ctx {
//     pub fn new() -> Ctx {
//         Ctx { counter: 0 }
//     }

//     pub fn get_counter(&mut self) -> u64 {
//         let c = self.counter;
//         self.counter += 1;
//         c
//     }

//     pub fn perform_calculation(&mut self, my_num: u64) -> u64 {
//         self.get_counter() + my_num
//     }
// }

// pub fn perform_calculation(ctx: &mut Ctx, my_num: u64) -> u64 {
//     ctx.get_counter() + my_num
// }
