extern crate vulkust;
extern crate rand;

use self::vulkust::math::vector::Vec3;
use self::vulkust::math::matrix::Mat3x3;

use self::rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let v = Vec3 { x: rng.gen::<f64>(), y: rng.gen(), z: rng.gen()}.normalized();
    println!("vector: {:?}", v);
    let d = rng.gen();
    println!("degree: {:?}", d);
    let m = Mat3x3::rotation(d, &v);
    println!("rotation matrix: {:?}", m);
}
