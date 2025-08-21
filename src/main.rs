use std::{io::Cursor, path::PathBuf, str::FromStr};

use serde_derive::{Deserialize, Serialize};

pub mod consts;
pub mod error;
pub mod utils;

use consts::*;
use error::*;
use utils::*;

#[derive(Serialize, Deserialize)]
pub struct Plugin {
    pub name: String,
    pub revision: usize,
    pub website: String,
    pub vendor: String,
    pub categories: Vec<String>,
    pub description: Option<String>,
    #[serde(alias = "instance")]
    pub instances: Vec<Instance>,
}

#[derive(Serialize, Deserialize)]
pub struct Instance {
    pub method: DistributionMethod,
    pub format: String,
    pub platform: String,
    pub arch: String,
}

#[derive(Serialize, Deserialize)]
pub enum DistributionMethod {
    SimpleDownloadAndCopy { downloads: Vec<DownloadAndCopyFile> },
    BehindNewsletter { url: String },
    BehindPayWhatYouWant { url: String },
    ProvidedInstaller { url: String },
}

#[derive(Serialize, Deserialize)]
pub struct DownloadAndCopyFile {
    pub url: String,
    pub extract: ExtractMethod,
    pub copy_actions: Vec<CopyAction>,
}

#[derive(Serialize, Deserialize)]
pub struct CopyAction(String, String);

#[derive(Serialize, Deserialize)]
pub enum ExtractMethod {
    None,
    Zip,
}

impl DownloadAndCopyFile {
    pub fn install(&self) -> Result<()> {
        let path = "temp";

        assert!(!PathBuf::from(path).exists());

        let res = (|| -> Result<()> {
            let mut call = ureq::get(&self.url).call().unwrap();
            let body = call.body_mut().read_to_vec().unwrap();

            match self.extract {
                ExtractMethod::None => {
                    std::fs::write(path, body).unwrap();
                }
                ExtractMethod::Zip => {
                    let c = Cursor::new(body);
                    let mut archive = zip::ZipArchive::new(c).unwrap();
                    archive.extract(path).unwrap();
                }
            }

            for CopyAction(src, dst) in &self.copy_actions {
                let src = process_path(src, path)?;
                let dst = process_path(dst, path)?;

                println!("Copying {} -> {}", src.display(), dst.display());

                if let Some(p) = dst.parent() {
                    if !p.exists() {
                        std::fs::create_dir_all(p).map_err(|e| Err::Misc(e.to_string()))?;
                    }
                }

                if src.is_dir() {
                    copy_dir(src, dst).map_err(|e| Err::Misc(e.to_string()))?;
                } else {
                    std::fs::copy(src, dst).map_err(|e| Err::Misc(e.to_string()))?;
                }
            }

            Ok(())
        })();

        std::fs::remove_dir_all(path).unwrap();

        res
    }
}

fn process_path(p: &str, dl: &str) -> Result<PathBuf> {
    let p = p
        .to_string()
        .replace("%dl%", dl)
        .replace("%vst2%", VST2_INSTALL_PATH)
        .replace("%vst3%", VST3_INSTALL_PATH)
        .replace("%clap%", CLAP_INSTALL_PATH);

    PathBuf::from_str(&p).map_err(|e| Err::Misc(e.to_string()))
}

impl Plugin {
    pub fn install(&self, _progress_callback: Option<Box<dyn Fn(f32)>>) -> Result<()> {
        let insts = self
            .instances
            .iter()
            .filter(|i| i.platform == THIS_PLATFORM);

        if insts.clone().count() == 0 {
            eprintln!("No instances of this plugin are avaible for this platform");
        }

        for inst in insts {
            match &inst.method {
                DistributionMethod::SimpleDownloadAndCopy { downloads } => {
                    for dl in downloads {
                        dl.install()?;
                    }
                }
                DistributionMethod::BehindNewsletter { url } => todo!(),
                DistributionMethod::ProvidedInstaller { url } => todo!(),
                DistributionMethod::BehindPayWhatYouWant { url } => todo!(),
            }
        }

        Ok(())
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() < 3 {
        eprintln!("Invalid usage");
        std::process::exit(1);
    }

    if args[1] == "install" {
        for plug in &args[2..] {
            let t = std::fs::read_to_string(plug).unwrap();
            let plug: std::result::Result<Plugin, _> = toml::from_str(&t);

            if let Err(ref e) = plug {
                println!("{}", e);
            }

            let plug = plug.unwrap();

            plug.install(None).unwrap();
        }
    } else {
        eprintln!("Unknown command {}", args[0]);
        std::process::exit(1);
    }
}
