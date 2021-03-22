extern crate app;

#[allow(unused_imports)]
use app::{App, Args, Cmd, Opt, OptTypo, OptValue, OptValueParse};
pub struct Cli {}

impl Cli {
    pub fn new() -> Cli {
        return Cli {};
    }

    pub fn run(&self) {
        Config::parse_args();
    }
}

#[derive(Debug, Default)]
pub struct Config {
    pub genesis_reward_addr: String,
    pub balance_addr: String,
    pub list_addr_limit: usize,
}

impl Config {
    fn parse_args() {
        let mut config = Config::default();

        let helper = {
            App::new("simplechain")
                .version("0.0.1")
                .author("MathxH Chen", "brainfvck@foxmail.com")
                .addr("Github", "https://github.com/AlexiaChen/simple-chain")
                .desc("A simple PoW-based blockchain.")
                .cmd(
                    Cmd::new("createblockchain")
                    .desc("Create a blockchain and send genesis block reward to <address>")
                    .opt(
                        Opt::new("address", &mut config.genesis_reward_addr)
                        .short('a')
                        .long("address")
                        .help("reward to address"),
                    )
                )
                .cmd(
                    Cmd::new("makekeypair")
                    .desc("Create new key pair and import it to wallet db")
                )
                .cmd(
                    Cmd::new("getbalance")
                    .desc("Get balance of <address>")
                    .opt(
                        Opt::new("address", &mut config.balance_addr)
                        .short('a')
                        .long("address")
                        .help("balance of address")
                    )
                )
                .cmd(
                    Cmd::new("listaddress")
                    .desc("list addresses from wallet db")
                    .opt(
                        Opt::new("limit", &mut config.list_addr_limit)
                        .short('n')
                        .long("num")
                        .help("addresses shown max number")
                    )
                )
                .cmd(
                    Cmd::new("getblock")
                )
                .cmd(
                    Cmd::new("gettx")
                )
                .cmd(
                    Cmd::new("sendtx")
                )
                .cmd(
                    Cmd::new("startnode")
                )
                .parse_args()
        };

        if *helper.args_len() == 0 {
            helper.help_exit(0);
        }

        config
            .check_and_call(helper.current_cmd_str())
            .map_err(|e| {
                helper.help_cmd_err_exit(helper.current_cmd_ref(), e, 1)
            })
            .unwrap(); // map_err alrendy exit if it is err, so unwrap is safe.
    }

    fn check_and_call(self, cmd: Option<&str>) -> Result<(), String> {
        println!("Match Cmd: {:?}", cmd);
        match cmd {
            Some("zip") => {}
            Some("ping") => {}
            Some("url") => {}
            _ => unreachable!(),
        }
        Ok(())
    }
}
