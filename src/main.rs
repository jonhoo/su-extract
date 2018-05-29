extern crate clap;
extern crate csv;
extern crate indicatif;
extern crate reqwest;
extern crate serde_json;
extern crate shlex;
extern crate url;

use std::io;

fn main() {
    eprintln!("Please paste the URL as described in the README:");

    let mut input = String::new();
    if let Err(e) = io::stdin().read_line(&mut input) {
        eprintln!("hmm, that doesn't seem right: {}", e);
        return;
    }

    if input.is_empty() {
        return;
    }

    // we'll let clap do the parsing for us:
    let args = match shlex::split(&input) {
        Some(args) => args,
        None => {
            return main();
        }
    };

    let curl = clap::App::new("cURLish")
        .version("0.1")
        .arg(
            clap::Arg::with_name("header")
                .short("H")
                .takes_value(true)
                .multiple(true),
        )
        .arg(clap::Arg::with_name("compressed").long("compressed"))
        .arg(clap::Arg::with_name("URL").required(true).index(1))
        .get_matches_from(args);

    // replicate all the headers
    let mut headers = reqwest::header::Headers::new();
    for header in curl.values_of("header").unwrap() {
        let mut parts = header.splitn(2, ": ");
        let name = parts.next().unwrap();
        let value = parts.next().unwrap();
        headers.set_raw(name.to_owned(), value.to_owned());
    }
    let client = reqwest::ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap();

    // now parse out things form the url
    let mut url: url::Url = curl.value_of("URL").unwrap().parse().unwrap();

    // start out with a base that only has the user's id and a high limit
    let uid = url
        .query_pairs()
        .find(|(name, _)| name == "userid")
        .unwrap()
        .1
        .to_string();
    let limit = 100;
    url.query_pairs_mut()
        .clear()
        .append_pair("userid", &uid)
        .append_pair("limit", &format!("{}", limit));

    // write out csv
    let mut w = csv::Writer::from_path("likes.csv").unwrap();

    // also show progress
    let mut pb = None;

    for page in 0.. {
        // make a url for the right page
        let mut url = url.clone();
        url.query_pairs_mut()
            .append_pair("offset", &format!("{}", page * limit));

        // fetch and parse the json
        let items = client
            .get(url)
            .send()
            .unwrap()
            .json::<serde_json::Map<String, serde_json::Value>>()
            .unwrap();

        let likes = match items.get("likes") {
            Some(&serde_json::Value::Object(ref o)) => o,
            Some(v) => {
                eprintln!("got unexpected contents in 'likes'");
                eprintln!("{:?}", v);
                break;
            }
            None => {
                eprintln!("no more likes in");
                eprintln!("{:?}", items);
                break;
            }
        };

        if pb.is_none() {
            if let Some(&serde_json::Value::Number(ref total)) = likes.get("_total") {
                let total = total.as_u64().unwrap();
                let mut bar = indicatif::ProgressBar::new(total);
                bar.set_style(
                    indicatif::ProgressStyle::default_bar()
                        .template(
                            "{spinner:.green} \
                             [{elapsed_precise}] \
                             [{bar:40.cyan/blue}] \
                             {pos}/{len} \
                             ({eta})",
                        )
                        .progress_chars("#>-"),
                );
                pb = Some(bar);
            }
        }

        let likes = match likes.get("values") {
            Some(&serde_json::Value::Array(ref a)) => a,
            _ => {
                eprintln!("got unexpected contents in 'likes' (bad .values)");
                eprintln!("{:?}", likes);
                break;
            }
        };

        if likes.is_empty() {
            // we're all done!
            break;
        }

        let mut n = 0;
        for liked in likes {
            match (liked.get("title"), liked.get("url")) {
                (
                    Some(&serde_json::Value::String(ref title)),
                    Some(&serde_json::Value::String(ref url)),
                ) => {
                    use std::iter;
                    w.write_record(iter::once(title).chain(iter::once(url)))
                        .unwrap();
                    n += 1;
                }
                _ => {}
            }
        }

        w.flush().unwrap();
        if let Some(ref mut pb) = pb {
            pb.inc(n);
        }
    }

    if let Some(pb) = pb {
        pb.finish();
    }
}
