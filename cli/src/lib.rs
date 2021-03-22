extern crate app;

#[allow(unused_imports)]
use app::{App, Args, Cmd, Opt, OptTypo, OptValue, OptValueParse};
use std::env;
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
    pub miner_addr: String,
    pub node_id: String,
    pub send_tx_from: String,
    pub send_tx_to: String,
    pub send_tx_amount: Vec<i32>,
    pub get_block_height: u32,
    pub get_block_hash: String,
    pub get_tx_id: Vec<String>,
}

impl Config {
    fn parse_args() {
        let mut config = Config::default();
        config.node_id = env::var("NODE_ID").unwrap_or("null".to_string());
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
                    .desc("get block info by height or hash")
                    .opt(
                        Opt::new("height", &mut config.get_block_height)
                        .short('h')
                        .long("height")
                        .help("block height")
                    )
                    .opt(
                        Opt::new("hash", &mut config.get_block_hash)
                        .short('i')
                        .long("hash")
                        .help("block hash")
                    )
                )
                .cmd(
                    Cmd::new("gettx")
                    .desc("get transaction info by tx hash")
                    .args(
                        Args::new("TXID", &mut config.get_tx_id)
                        .help("tx id or tx hash")
                    )

                )
                .cmd(
                    Cmd::new("sendtx")
                    .desc("send tx from address to another address")
                    .opt(
                        Opt::new("from", &mut config.send_tx_from)
                        .short('f')
                        .long("from")
                        .help("tx from address")
                    )
                    .opt(
                        Opt::new("to", &mut config.send_tx_to)
                        .short('t')
                        .long("to")
                        .help("tx to address")
                    )
                    .args(
                        Args::new("AMOUNT", &mut config.send_tx_amount)
                        .help("token amount")
                    )
                )
                .cmd(
                    Cmd::new("startnode")
                    .desc("start node server")
                    .opt(
                        Opt::new("miner", &mut config.miner_addr)
                        .long("miner")
                        .help("miner address")
                    )
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
            Some("createblockchain") => {}
            Some("makekeypair") => {}
            Some("getbalance") => {}
            Some("listaddress") => {}
            Some("getblock") => {}
            Some("gettx") => {}
            Some("sendtx") => {}
            Some("startnode") => {}
            _ => unreachable!(),
        }
        Ok(())
    }
}
