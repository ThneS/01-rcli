use anyhow::Ok;
use clap::Parser;
use enum_dispatch::enum_dispatch;

use crate::{process_jwt_sign, process_jwt_verify, CmdExector};
#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum JwtSubCommand {
    #[command(about = "Sign a JWT token")]
    Sign(JwtSignOpts),
    #[command(about = "Verify a JWT token")]
    Verify(JwtVerifyOpts),
}

#[derive(Debug, Parser)]
pub struct JwtSignOpts {
    #[arg(short, long, help = "Subject")]
    sub: String,
    #[arg(short, long, help = "Audience")]
    aud: String,
    #[arg(short, long, help = "Expiration time in seconds")]
    exp: String,
}

#[derive(Debug, Parser)]
pub struct JwtVerifyOpts {
    #[arg(short, long, help = "JWT token")]
    token: String,
}
impl CmdExector for JwtVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        process_jwt_verify(&self.token)
    }
}
impl CmdExector for JwtSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let res = process_jwt_sign(&self.sub, &self.aud, &self.exp)?;
        println!("{}", res);
        Ok(())
    }
}
