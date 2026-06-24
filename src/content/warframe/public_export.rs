use std::env::temp_dir;
use std::{env, fs};
use std::path::PathBuf;
use crate::content::warframe::language_code::LanguageCode;
use crate::meta::tools::{download, extract};
use cortex::Format;

pub struct PublicExport {
    pub language_code: LanguageCode,
    pub index_url: String,
    pub contents: String,
}

impl PublicExport {
    pub fn new(language_code: LanguageCode) -> Self {
        let testing = true;

        let index_url = format!(
            "https://origin.warframe.com/PublicExport/index_{}.txt.lzma",
            language_code
        );

        let mut output_path: PathBuf = PathBuf::from(env::var("WARFRAME_PATH").unwrap()).join("Public_Export/");

        if (testing) {
            output_path = temp_dir().join("warframe-public-export/");
        }

        // let lzma_file = Format::Lzma::new("Download");
        //
        // download(index_url.as_str(), &file).expect("failed to download");

        download(index_url.as_str(), "download.lzma").expect("failed to download");
        extract("download.lzma").expect("failed to extract file");

        let contents = fs::read_to_string("download").expect("failed to read file");
        println!("{}", contents);



        Self { language_code, index_url, contents }
    }
}

