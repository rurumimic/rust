use art::mix; // <- use art::kinds::PrimaryColor;
use art::PrimaryColor; // <- use art::utils::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
