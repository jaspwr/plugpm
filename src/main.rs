use std::{collections::HashMap, io::Cursor, path::PathBuf, str::FromStr};

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
    #[serde(default)]
    pub version: String,
    pub revision: usize,
    pub website: String,
    pub vendor: String,
    #[serde(default)]
    pub licence: String,
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

#[derive(Default)]
pub struct Context {
    /// URL -> Path
    downloaded: HashMap<String, String>,
}

impl DownloadAndCopyFile {
    pub fn install(&self, fmt: &str, ctx: &mut Context) -> Result<()> {
        // Check for existing download
        let path = match ctx.downloaded.get(&self.url) {
            Some(p) => p.clone(),
            None => {
                let path = slugify(&self.url);
                self.download(&path)?;
                ctx.downloaded.insert(self.url.clone(), path.clone());
                path
            }
        };

        for CopyAction(src, dst) in &self.copy_actions {
            let src = process_path(src, &path, fmt)?;
            let dst = process_path(dst, &path, fmt)?;

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
    }

    fn download(&self, path: &str) -> Result<()> {
        let mut call = ureq::get(&self.url).call().unwrap();
        let body = call.body_mut().read_to_vec().unwrap();

        assert!(!PathBuf::from(path).exists());

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

        Ok(())
    }
}

fn process_path(p: &str, dl: &str, fmt: &str) -> Result<PathBuf> {
    let p = p.to_string().replace("%dl%", dl).replace(
        "%plugs%",
        match fmt {
            FORMAT_VST2 => VST2_INSTALL_PATH,
            FORMAT_VST3 => VST3_INSTALL_PATH,
            FORMAT_CLAP => CLAP_INSTALL_PATH,
            _ => {
                return Err(Err::Misc(
                    "No install path known for this format".to_string(),
                ));
            }
        },
    );

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

        let mut ctx = Context::default();

        for inst in insts {
            match &inst.method {
                DistributionMethod::SimpleDownloadAndCopy { downloads } => {
                    for dl in downloads {
                        if let Err(e) = dl.install(&inst.format, &mut ctx) {
                            eprintln!("[!] {} {}", inst.format, e);
                        }
                    }
                }
                DistributionMethod::BehindNewsletter { url } => todo!(),
                DistributionMethod::ProvidedInstaller { url } => todo!(),
                DistributionMethod::BehindPayWhatYouWant { url } => todo!(),
            }
        }

        for (_, path) in ctx.downloaded {
            std::fs::remove_dir_all(path).unwrap();
        }

        Ok(())
    }
}

fn fetch(name: &str) -> Result<Plugin> {
    let url = format!(
        "https://raw.githubusercontent.com/jaspwr/plugpm/refs/heads/main/pkgs/{}.toml",
        name
    );
    let mut call = ureq::get(url).call().unwrap();
    let body = call.body_mut().read_to_string().unwrap();
    let plug: std::result::Result<Plugin, _> = toml::from_str(&body);
    if let Err(ref e) = plug {
        println!("{}", e);
    }
    let plug = plug.unwrap();
    Ok(plug)
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() < 3 {
        eprintln!("Invalid usage");
        std::process::exit(1);
    }

    if args[1] == "install" {
        for plug in &args[2..] {
            let plug = fetch(&plug).unwrap();
            plug.install(None).unwrap();
        }
    } else {
        eprintln!("Unknown command {}", args[0]);
        std::process::exit(1);
    }
}
