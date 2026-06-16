use anyhow::Result;
use std::fs;
use std::path::Path;

pub static LOGO_PNG: &[u8]     = include_bytes!("imgs/CaronteSoftware.png");
pub static LOGOMARCA_PNG: &[u8] = include_bytes!("imgs/logomarca.png");
pub static FAVICON_ICO: &[u8]  = include_bytes!("imgs/iconeweb.ico");

/// Extrai os assets de marca em `<dest_dir>/caronte/`:
///   CaronteSoftware.png, logomarca.png, iconeweb.ico
/// e, no Linux, cria uma entrada .desktop em ~/.local/share/applications/.
pub fn install_assets(dest_dir: &str) -> Result<()> {
    let assets_dir = Path::new(dest_dir).join("caronte");
    fs::create_dir_all(&assets_dir)?;

    let files: &[(&str, &[u8])] = &[
        ("CaronteSoftware.png", LOGO_PNG),
        ("logomarca.png",       LOGOMARCA_PNG),
        ("iconeweb.ico",        FAVICON_ICO),
    ];

    for (name, data) in files {
        let dest = assets_dir.join(name);
        fs::write(&dest, data)?;
        println!("  [caronte] asset instalado: {}", dest.display());
    }

    #[cfg(target_os = "linux")]
    create_desktop_entry(&assets_dir)?;

    Ok(())
}

#[cfg(target_os = "linux")]
fn create_desktop_entry(assets_dir: &Path) -> Result<()> {
    let icon_path = assets_dir.join("CaronteSoftware.png");

    let desktop = format!(
        "[Desktop Entry]\n\
         Version=1.0\n\
         Type=Application\n\
         Name=Caronte\n\
         Comment=Gerador de código Laravel / CI4 / Landing Page\n\
         Exec=caronte\n\
         Icon={}\n\
         Terminal=true\n\
         Categories=Development;\n",
        icon_path.display()
    );

    let apps_dir = dirs_home()?.join(".local/share/applications");
    std::fs::create_dir_all(&apps_dir)?;
    let entry = apps_dir.join("caronte.desktop");
    std::fs::write(&entry, desktop)?;
    println!("  [caronte] .desktop criado: {}", entry.display());

    Ok(())
}

#[cfg(target_os = "linux")]
fn dirs_home() -> Result<std::path::PathBuf> {
    std::env::var("HOME")
        .map(std::path::PathBuf::from)
        .map_err(|_| anyhow::anyhow!("Variavel HOME nao encontrada"))
}
