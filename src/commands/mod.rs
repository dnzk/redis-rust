mod info;
mod set;
mod set_arguments;
pub use info::Info;
pub use set::Set;

pub trait Resp {
    fn reply(&self) -> String;
}
