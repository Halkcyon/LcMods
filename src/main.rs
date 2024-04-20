use std::fs;
use std::io::{self, prelude::*, StdinLock, StdoutLock};
use std::path::{Path, PathBuf};
use zip::read::ZipArchive;

const MODS_ZIP: &'static [u8] = include_bytes!("mods.zip");

fn main() {
    let mut stdout = stdout();
    let mut stdin = stdin();

    let steam_root = prompt_for_steam_library_root(&mut stdout, &mut stdin);
    let Some(steam_root) = steam_root else {
        exit("could not find provided SteamLibrary", &mut stdin);
    };

    let lc_root = get_lethal_company_root(&steam_root);
    let Some(lc_root) = lc_root else {
        exit("could not find `Lethal Company` path in the provided SteamLibrary", &mut stdin);
    };

    let crsr = io::Cursor::new(MODS_ZIP);
    let mut zip = ZipArchive::new(crsr).unwrap();

    let mods_root = lc_root.join("BepInExPack");
    if mods_root.exists() {
        if let Err(err) = fs::remove_dir_all(&mods_root) {
            exit(&format!("failed to remove existing mods: {err}"), &mut stdin);
        }
    }

    if let Err(err) = zip.extract(&lc_root) {
        exit(&format!("failed to extract mods: {err}"), &mut stdin);
    };
}

fn get_lethal_company_root(steam_root: &Path) -> Option<PathBuf> {
    let path = steam_root.join("steamapps/common/Lethal Company");

    match path.try_exists() {
        Ok(exists) => {
            if exists {
                return Some(path);
            }
        }
        Err(err) => {
            eprintln!("access error to `Lethal Company`: {err:?}");
        }
    }

    None
}

fn prompt_for_steam_library_root(
    stdout: &mut StdoutLock,
    stdin: &mut StdinLock,
) -> Option<PathBuf> {
    stdout
        .write(b"Enter the path to your SteamLibrary where Lethal Company is installed\r\n")
        .unwrap();
    stdout
        .write(b"e.g., `C:\\Program Files (x86)\\Steam`, `G:\\SteamLibrary`\r\n")
        .unwrap();
    stdout.flush().unwrap();

    let mut user_input = String::new();
    stdin
        .read_line(&mut user_input)
        .expect("unable to read user input");
    let user_input = user_input.trim();

    let path = PathBuf::from(user_input);
    match path.try_exists() {
        Ok(exists) => {
            if exists {
                return Some(path);
            }
        }
        Err(err) => {
            eprintln!("access error to SteamLibrary: {err:?}");
        }
    }

    None
}

fn exit(message: &str, stdin: &mut StdinLock) -> ! {
    eprintln!("{message}");

    eprintln!("Press ENTER to exit.");
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    std::process::exit(1);
}

fn stdin() -> StdinLock<'static> {
    let stdin = io::stdin();

    stdin.lock()
}

fn stdout() -> StdoutLock<'static> {
    let stdout = io::stdout();

    stdout.lock()
}
