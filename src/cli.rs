use chrono;
use clap::Clap;
use crate::star;

#[derive(Clap)]
#[clap(name = "bilibili-anime", version = "0.1")]
struct CliArgs {
  #[clap(short, long, about = "展示特定日期番剧 YYYY-MM-DD")]
  date: Option<String>,

  #[clap(short, long, about = "仅展示收藏番剧")]
  star: bool,

  #[clap(short, long, about = "仅展示当天播出番剧")]
  today: bool,

  #[clap(subcommand)]
  subcmd: Option<SubCommand>,
}

#[derive(Clap)]
enum SubCommand {
  #[clap(about = "操作收藏番剧")]
  Star(StarCommand)
}

#[derive(Clap)]
struct StarCommand {
  #[clap(subcommand)]
  subcmd: StarSubCommand,
}

#[derive(Clap)]
enum StarSubCommand {
  #[clap(about = "收藏番剧")]
  Add(StarName),
  #[clap(about = "移除收藏番剧")]
  Remove(StarName),
  #[clap(about = "收藏番剧列表")]
  List,
  #[clap(about = "清空收藏番剧")]
  Clean,
}

#[derive(Clap)]
struct StarName {
  name: String,
}

pub struct Options {
  pub date: Option<i64>,
  pub star: bool,
}

pub fn parse_cli() -> Option<Options> {
  let cli_args = CliArgs::parse();
  match cli_args.subcmd {
    Some(SubCommand::Star(s)) => {
      match &s.subcmd {
        StarSubCommand::Add(star_add) => star::add(&star_add.name),
        StarSubCommand::Remove(star_remove) => star::remove(&star_remove.name),
        StarSubCommand::List => star::list(),
        StarSubCommand::Clean => star::clean(),
      }
      return None;
    }
    _ => (),
  };
  Some(Options {
    date: if let Some(date) = &cli_args.date {
      Some(
        chrono::DateTime::parse_from_str(
          &format!("{} {}", date, "00:00:00 +08:00"),
          "%Y-%m-%d %H:%M:%S %z",
        )
        .expect("parse date error")
        .timestamp(),
      )
    } else if cli_args.today {
      Some(chrono::Local::today().and_hms(0, 0, 0).timestamp())
    } else {
      None
    },
    star: cli_args.star,
  })
}
