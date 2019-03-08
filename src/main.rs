use reqwest;
use serde::Deserialize;
use std::collections::HashMap;
use std::time::Duration;
use structopt::StructOpt;

mod errors {
    use error_chain::*;
    error_chain! {}
}

use errors::*;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(long)]
    api_key: String,
    #[structopt(long)]
    slug: String,
    #[structopt(long, short = "t", default_value = "full")]
    gen_type: String,
}

fn main() {
    if let Err(ref e) = run() {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}

#[derive(Deserialize, Debug)]
struct JobStatus {
    message: String,
}

#[derive(PartialEq)]
enum GenType {
    Subset,
    Full,
}

fn run() -> Result<()> {
    let opt = Opt::from_args();

    let gen_type = if opt.gen_type == "subset" {
        GenType::Subset
    } else {
        GenType::Full
    };

    let client = reqwest::Client::builder()
        .gzip(true)
        .timeout(Duration::from_secs(120))
        .build()
        .chain_err(|| "Unable to create HTTP client.")?;

    start_preview_job(&client, &opt.slug, &opt.api_key, gen_type)?;
    wait_until_done(&client, &opt.slug, &opt.api_key)?;
    download_files(&client, &opt.slug, &opt.api_key)
}

fn start_preview_job(
    client: &reqwest::Client,
    slug: &str,
    api_key: &str,
    gen_type: GenType,
) -> Result<()> {
    let preview_url = if gen_type == GenType::Subset {
        format!("https://leanpub.com/{}/preview/subset.json", slug)
    } else {
        format!("https://leanpub.com/{}/preview.json", slug)
    };
    let mut params = HashMap::new();
    params.insert("api_key", api_key.clone());
    client
        .post(&preview_url)
        .form(&params)
        .send()
        .chain_err(|| "Unable to start preview job.")?;
    Ok(())
}

fn wait_until_done(client: &reqwest::Client, slug: &str, api_key: &str) -> Result<()> {
    let status_url = format!(
        "https://leanpub.com/{}/job_status.json?api_key={}",
        &slug, &api_key
    );
    let mut last_status_msg = "".to_string();
    loop {
        let mut resp = client
            .get(&status_url)
            .send()
            .chain_err(|| "Unable to get job status.")?;
        if resp.status().is_success() {
            match resp.json::<JobStatus>() {
                Ok(status) => {
                    if last_status_msg != status.message {
                        println!("{}", &status.message);
                        last_status_msg = status.message
                    }
                }
                // When the job is done. The server returns {}, so parsing will fail.
                _ => return Ok(()),
            }
        } else {
            return Err(Error::from(format!("Server returned: {}", resp.status())));
        }
        std::thread::sleep(Duration::from_millis(3000));
    }
}

#[derive(Deserialize, Debug)]
struct BookInfo {
    pdf_preview_url: String,
    epub_preview_url: String,
    mobi_preview_url: String,
}

fn download_files(client: &reqwest::Client, slug: &str, api_key: &str) -> Result<()> {
    let info_url = format!("https://leanpub.com/{}.json?api_key={}", slug, api_key);
    let mut resp = client
        .get(&info_url)
        .send()
        .chain_err(|| "Unable to get book info.")?;
    if resp.status().is_success() {
        match resp.json::<BookInfo>() {
            Ok(book) => {
                download_file(
                    client,
                    &book.pdf_preview_url,
                    &format!("{}-preview.pdf", slug),
                )?;
                download_file(
                    client,
                    &book.epub_preview_url,
                    &format!("{}-preview.epub", slug),
                )?;
                download_file(
                    client,
                    &book.mobi_preview_url,
                    &format!("{}-preview.mobi", slug),
                )
            }
            _ => Err(Error::from("Failed to parse server response.")),
        }
    } else {
        return Err(Error::from(format!("Server returned: {}", resp.status())));
    }
}

fn download_file(client: &reqwest::Client, url: &str, fname: &str) -> Result<()> {
    println!("Downloading {}", fname);
    let mut file =
        std::fs::File::create(fname).chain_err(|| format!("Cannot create {}.", fname))?;
    let mut response = client
        .get(url)
        .send()
        .chain_err(|| format!("Failed to download from {}.", url))?;
    std::io::copy(&mut response, &mut file)
        .chain_err(|| format!("Unable to write to {}", fname))?;
    Ok(())
}
