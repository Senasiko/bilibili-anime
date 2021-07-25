mod anime;
mod cli;
mod request;
mod star;
use anime::Anime;

fn main() {
  let rt = tokio::runtime::Runtime::new().unwrap();
  rt.block_on(run()).unwrap();
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
  if let Some(options) = cli::parse_cli() {
    let day_result = request::request_bangumi().await?;

    let stars = if options.star {
      star::parse_stars()
    } else {
      vec![]
    };
    if let Some(date) = &options.date {
      let mut animes: Vec<&anime::Anime> =
        if let Some(seasons) = request::get_animes_by_date(&day_result, date) {
          seasons.to_vec()
        } else {
          vec![]
        };
      if options.star {
        animes = animes
          .into_iter()
          .filter(|anime| star::is_star(&stars, &anime.title))
          .collect::<Vec<&Anime>>();
      }
      for anime in animes.iter() {
        println!("{}", anime);
      }
    } else {
      for day in day_result.iter() {
        let mut result: Vec<&Anime> = vec![];
        for anime in day.seasons.iter() {
          if !options.star || star::is_star(&stars, &anime.title) {
            result.push(anime)
          }
        }
        if result.len() > 0 {
          println!("{}", day.date);
          for anime in result.iter() {
            println!("{}", anime);
          }
        }
      }
    };
  }
  Ok(())
}
