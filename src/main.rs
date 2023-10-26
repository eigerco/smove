use smove::smove_cli;

fn main() -> anyhow::Result<()> {
    let cwd = std::env::current_dir()?;

    smove_cli(cwd)
}
