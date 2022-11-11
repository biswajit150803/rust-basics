fn cal(mass:i32)->f32{
    let mut gravity:f32=10.0;
    gravity=9.81;
    (mass*gravity) as f32
}
fn main() {
    println!("Hello, world! {}",cal(5.2));
}
