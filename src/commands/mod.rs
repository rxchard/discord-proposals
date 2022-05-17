use serenity::framework::standard::macros::group;

pub mod init;
use init::*;

#[group]
#[commands(post_interact)]
pub struct General;
