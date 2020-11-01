use serenity::framework::standard::macros::group;
pub mod help;
pub use help::*;

pub mod obs;
pub use obs::*;
#[group]
#[commands(join, leave)]
struct Obs;

pub mod utility;
pub use utility::*;
#[group]
#[commands(bgrm)]
struct Utility;


