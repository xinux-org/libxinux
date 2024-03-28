pub mod any;
pub mod aur;
pub mod std;

pub enum Pkgs {
    Any(any::Any),
    Aur(aur::Aur),
    Std(std::Std),
}
