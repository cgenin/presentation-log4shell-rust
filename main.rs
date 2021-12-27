#[macro_use]
use std::fs::File;

use clap::Parser;
use walkdir::WalkDir;
use zip::ZipArchive;

#[derive(Parser)]
#[clap(version = "1.0", author = "Genin C.")]
struct Opts {
    /// Extension recherchée
    #[clap(short, long, default_value = "war")]
    extension: String,
    /// Répertoire de recherche
    #[clap(short, long, default_value = ".")]
    path: String,
    /// Chaine recherchée
    #[clap(short, long, default_value = "log4j-core")]
    search: String,
}

fn main() {
    let args: Opts = Opts::parse();
    let extension = args.extension.as_str();
    let search = args.search.as_str();
    let walker = WalkDir::new(args.path.as_str());
    for entry in walker
    {
        let entry = entry.unwrap();
        let path = entry.path();

        let path_string = path.display().to_string();
        let result = path_string.to_lowercase().ends_with(extension);
        if result {
            let file = File::open(path).expect("Impossible d'ouvrir le fichier");
            let mut archive = ZipArchive::new(file).expect("Impossible d'ouvrir l'archive zip");
            for i in 0..archive.len() {
                let mut file = archive.by_index(i).expect("Index non trouvé");
                let name = file.name();
                if name.contains(search) {
                    println!("{}:{}", path_string, name);
                }
            }
        }
    }
}
