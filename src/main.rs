pub mod gameboy;

fn main() {
    let mut derp = gameboy::cpu::new();
    derp.set_A(8);
    derp.set_F(8);
    println!("{}", derp.get_AF());
    derp.set_F(20);
    println!("{}", derp.get_AF());
}
