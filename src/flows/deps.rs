// ── Verificação e instalação automática de dependências ──────────────────────
//
// Módulo compartilhado por todos os flows de geração de arquitetura DDD.
// Verifica (e tenta instalar automaticamente) PHP LTS, Composer e PostgreSQL.

#![allow(dead_code)]

use anyhow::{bail, Result};
use console::style;
use std::fs;
use std::process::Command;

// ── Detecção de SO ────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq)]
pub enum Os {
    Windows,
    Macos,
    Debian,
    Fedora,
    Unknown,
}

pub fn detect_os() -> Os {
    if cfg!(target_os = "windows") {
        return Os::Windows;
    }
    if cfg!(target_os = "macos") {
        return Os::Macos;
    }
    if Command::new("apt-get").arg("--version").output().is_ok() {
        return Os::Debian;
    }
    if Command::new("dnf").arg("--version").output().is_ok() {
        return Os::Fedora;
    }
    Os::Unknown
}

// ── PHP ───────────────────────────────────────────────────────────────────────

fn php_version_string() -> Option<String> {
    let out = Command::new("php").arg("--version").output().ok()?;
    if !out.status.success() {
        return None;
    }
    let line = String::from_utf8_lossy(&out.stdout);
    line.lines()
        .next()
        .and_then(|l| l.split_whitespace().nth(1))
        .map(|s| s.to_string())
}

fn php_is_ok(version: &str) -> bool {
    let parts: Vec<u32> = version
        .split('.')
        .filter_map(|p| p.parse().ok())
        .collect();
    let major = parts.first().copied().unwrap_or(0);
    let minor = parts.get(1).copied().unwrap_or(0);
    major > 8 || (major == 8 && minor >= 2)
}

fn install_php(os: &Os) -> Result<()> {
    println!(
        "  {} Instalando PHP 8.3 LTS + todas as extensões...",
        style("→").cyan()
    );

    let ok = match os {
        Os::Debian => {
            let _ = Command::new("sudo")
                .args(["apt-get", "install", "-y", "software-properties-common"])
                .status();
            let _ = Command::new("sudo")
                .args(["add-apt-repository", "-y", "ppa:ondrej/php"])
                .status();
            let _ = Command::new("sudo")
                .args(["apt-get", "update", "-y"])
                .status();

            Command::new("sudo")
                .args([
                    "apt-get", "install", "-y",
                    // runtime principal
                    "php8.3", "php8.3-cli", "php8.3-fpm", "php8.3-common",
                    // strings & encoding
                    "php8.3-mbstring", "php8.3-xml", "php8.3-dom", "php8.3-xsl",
                    // rede
                    "php8.3-curl",
                    // arquivo & filesystem
                    "php8.3-zip", "php8.3-phar", "php8.3-fileinfo",
                    // matemática & criptografia
                    "php8.3-bcmath", "php8.3-gmp",
                    // blade tokenizer
                    "php8.3-tokenizer",
                    // bancos de dados
                    "php8.3-mysql", "php8.3-pgsql", "php8.3-sqlite3",
                    // cache & sessões
                    "php8.3-redis", "php8.3-apcu",
                    // imagem
                    "php8.3-gd", "php8.3-imagick",
                    // internacionalização
                    "php8.3-intl",
                    // web services
                    "php8.3-soap",
                    // LDAP & e-mail
                    "php8.3-ldap", "php8.3-imap",
                    // performance
                    "php8.3-opcache",
                    // shell interativo
                    "php8.3-readline",
                ])
                .status()
                .map(|s| s.success())
                .unwrap_or(false)
        }
        Os::Fedora => {
            Command::new("sudo")
                .args([
                    "dnf", "install", "-y",
                    "php", "php-cli", "php-fpm", "php-common",
                    "php-mbstring", "php-xml", "php-dom",
                    "php-curl",
                    "php-zip", "php-fileinfo",
                    "php-bcmath", "php-gmp",
                    "php-mysqlnd", "php-pgsql", "php-sqlite3",
                    "php-redis", "php-pecl-apcu",
                    "php-gd", "php-pecl-imagick",
                    "php-intl",
                    "php-soap",
                    "php-opcache",
                ])
                .status()
                .map(|s| s.success())
                .unwrap_or(false)
        }
        Os::Macos => {
            let ok = Command::new("brew")
                .args(["install", "php"])
                .status()
                .map(|s| s.success())
                .unwrap_or(false);
            if ok {
                for ext in &["redis", "imagick", "apcu"] {
                    let _ = Command::new("pecl").args(["install", ext]).status();
                }
            }
            ok
        }
        Os::Windows => Command::new("winget")
            .args(["install", "--id", "PHP.PHP", "-e", "--silent"])
            .status()
            .map(|s| s.success())
            .unwrap_or(false),
        Os::Unknown => false,
    };

    if !ok {
        bail!(
            "Não foi possível instalar o PHP 8.3 automaticamente.\n\
             Instale manualmente:\n\
             • Ubuntu/Debian : sudo apt install php8.3 php8.3-cli php8.3-fpm php8.3-mbstring \\\n\
                               php8.3-xml php8.3-curl php8.3-pgsql php8.3-mysql php8.3-redis \\\n\
                               php8.3-gd php8.3-intl php8.3-opcache php8.3-zip php8.3-bcmath\n\
             • Fedora/RHEL   : sudo dnf install php php-cli php-fpm php-mbstring php-xml \\\n\
                               php-curl php-pgsql php-mysqlnd php-redis php-gd php-intl php-opcache\n\
             • macOS (brew)  : brew install php && pecl install redis imagick apcu\n\
             • Windows       : winget install PHP.PHP\n\
             Documentação    : https://www.php.net/manual/en/install.php"
        );
    }
    Ok(())
}

/// Garante PHP ≥ 8.2 instalado com todas as extensões necessárias para Laravel.
/// Instala automaticamente se não encontrado.
pub fn ensure_php() -> Result<String> {
    if let Some(v) = php_version_string() {
        if php_is_ok(&v) {
            return Ok(v);
        }
        bail!(
            "PHP {v} detectado — Laravel 11 requer PHP ≥ 8.2.\n\
             Atualize para PHP 8.3 LTS ou superior."
        );
    }

    println!(
        "  {} PHP não encontrado — tentando instalar automaticamente...",
        style("!").yellow().bold()
    );
    let os = detect_os();
    install_php(&os)?;

    match php_version_string() {
        Some(v) if php_is_ok(&v) => {
            println!(
                "  {} PHP {} instalado com sucesso.",
                style("✔").green().bold(),
                style(&v).white().bold()
            );
            Ok(v)
        }
        Some(v) => bail!("PHP {v} instalado, mas é < 8.2 — atualize manualmente."),
        None => bail!(
            "PHP instalado mas não encontrado no PATH. Reinicie o terminal e tente novamente."
        ),
    }
}

// ── Composer ──────────────────────────────────────────────────────────────────

fn composer_available() -> bool {
    Command::new("composer")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn install_composer(os: &Os) -> Result<()> {
    println!("  {} Instalando Composer...", style("→").cyan());

    let ok = match os {
        Os::Windows => Command::new("winget")
            .args(["install", "--id", "Composer.Composer", "-e", "--silent"])
            .status()
            .map(|s| s.success())
            .unwrap_or(false),

        Os::Macos | Os::Debian | Os::Fedora | Os::Unknown => {
            let dl = Command::new("php")
                .args([
                    "-r",
                    "copy('https://getcomposer.org/installer', 'composer-setup.php');",
                ])
                .status()
                .map(|s| s.success())
                .unwrap_or(false);

            if !dl {
                return Ok(());
            }

            let _ = Command::new("php")
                .args(["composer-setup.php"])
                .status();
            let _ = fs::remove_file("composer-setup.php");

            Command::new("sudo")
                .args(["mv", "composer.phar", "/usr/local/bin/composer"])
                .status()
                .map(|s| s.success())
                .unwrap_or(false)
        }
    };

    if !ok {
        bail!(
            "Não foi possível instalar o Composer automaticamente.\n\
             Instale manualmente: https://getcomposer.org/download/"
        );
    }
    Ok(())
}

/// Garante que o Composer está disponível no PATH.
/// Instala automaticamente se não encontrado.
pub fn ensure_composer() -> Result<()> {
    if composer_available() {
        return Ok(());
    }
    println!(
        "  {} Composer não encontrado — tentando instalar automaticamente...",
        style("!").yellow().bold()
    );
    let os = detect_os();
    install_composer(&os)?;

    if !composer_available() {
        bail!(
            "Composer instalado mas não encontrado no PATH.\n\
             Reinicie o terminal e tente novamente, ou instale manualmente: https://getcomposer.org/download/"
        );
    }
    println!("  {} Composer instalado com sucesso.", style("✔").green().bold());
    Ok(())
}

// ── PostgreSQL ────────────────────────────────────────────────────────────────

fn postgres_available() -> bool {
    // verifica psql ou pg_isready
    Command::new("psql")
        .args(["--version"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or_else(|_| {
            Command::new("pg_isready")
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
        })
}

fn install_postgres(os: &Os) -> Result<()> {
    println!(
        "  {} Instalando PostgreSQL 16 LTS + extensões...",
        style("→").cyan()
    );

    let ok = match os {
        Os::Debian => {
            // adiciona repositório oficial PGDG para garantir versão 16
            let _ = Command::new("sudo")
                .args(["apt-get", "install", "-y", "curl", "ca-certificates"])
                .status();
            let _ = Command::new("bash")
                .args([
                    "-c",
                    "curl -fsSL https://www.postgresql.org/media/keys/ACCC4CF8.asc \
                     | sudo gpg --dearmor -o /etc/apt/trusted.gpg.d/postgresql.gpg",
                ])
                .status();
            let _ = Command::new("bash")
                .args([
                    "-c",
                    "echo 'deb https://apt.postgresql.org/pub/repos/apt \
                     $(. /etc/os-release; echo $VERSION_CODENAME)-pgdg main' \
                     | sudo tee /etc/apt/sources.list.d/pgdg.list",
                ])
                .status();
            let _ = Command::new("sudo")
                .args(["apt-get", "update", "-y"])
                .status();

            Command::new("sudo")
                .args([
                    "apt-get", "install", "-y",
                    "postgresql-16",
                    "postgresql-client-16",
                    "postgresql-contrib-16",
                    // extensões úteis para Laravel
                    "postgresql-16-pgvector",
                    "postgresql-16-cron",
                ])
                .status()
                .map(|s| s.success())
                .unwrap_or(false)
        }
        Os::Fedora => {
            let _ = Command::new("sudo")
                .args(["dnf", "install", "-y", "https://download.postgresql.org/pub/repos/yum/reporpms/EL-9-x86_64/pgdg-redhat-repo-latest.noarch.rpm"])
                .status();
            let _ = Command::new("sudo")
                .args(["dnf", "-qy", "module", "disable", "postgresql"])
                .status();

            Command::new("sudo")
                .args([
                    "dnf", "install", "-y",
                    "postgresql16-server",
                    "postgresql16",
                    "postgresql16-contrib",
                ])
                .status()
                .and_then(|s| {
                    if s.success() {
                        // inicializa o cluster
                        let _ = Command::new("sudo")
                            .args(["/usr/pgsql-16/bin/postgresql-16-setup", "initdb"])
                            .status();
                        let _ = Command::new("sudo")
                            .args(["systemctl", "enable", "--now", "postgresql-16"])
                            .status();
                    }
                    Ok(s)
                })
                .map(|s| s.success())
                .unwrap_or(false)
        }
        Os::Macos => Command::new("brew")
            .args(["install", "postgresql@16"])
            .status()
            .and_then(|s| {
                if s.success() {
                    let _ = Command::new("brew")
                        .args(["services", "start", "postgresql@16"])
                        .status();
                }
                Ok(s)
            })
            .map(|s| s.success())
            .unwrap_or(false),

        Os::Windows => Command::new("winget")
            .args([
                "install",
                "--id",
                "PostgreSQL.PostgreSQL",
                "-e",
                "--silent",
            ])
            .status()
            .map(|s| s.success())
            .unwrap_or(false),

        Os::Unknown => false,
    };

    if !ok {
        bail!(
            "Não foi possível instalar o PostgreSQL automaticamente.\n\
             Instale manualmente:\n\
             • Ubuntu/Debian : sudo apt install postgresql-16 postgresql-contrib-16\n\
             • Fedora/RHEL   : sudo dnf install postgresql16-server postgresql16-contrib\n\
             • macOS (brew)  : brew install postgresql@16 && brew services start postgresql@16\n\
             • Windows       : winget install PostgreSQL.PostgreSQL\n\
             Documentação    : https://www.postgresql.org/download/"
        );
    }
    Ok(())
}

/// Garante que o PostgreSQL está instalado e acessível via psql/pg_isready.
/// Instala automaticamente se não encontrado.
pub fn ensure_postgres() -> Result<()> {
    if postgres_available() {
        return Ok(());
    }
    println!(
        "  {} PostgreSQL não encontrado — tentando instalar automaticamente...",
        style("!").yellow().bold()
    );
    let os = detect_os();
    install_postgres(&os)?;

    if !postgres_available() {
        bail!(
            "PostgreSQL instalado mas não encontrado no PATH.\n\
             Reinicie o terminal e tente novamente, ou instale manualmente: https://www.postgresql.org/download/"
        );
    }
    println!(
        "  {} PostgreSQL instalado com sucesso.",
        style("✔").green().bold()
    );
    Ok(())
}

// ── Verificação completa para flows DDD ──────────────────────────────────────

/// Executa todas as verificações de dependência necessárias para geração de
/// código DDD/Laravel: PHP 8.3 LTS, Composer e PostgreSQL.
///
/// Retorna a versão do PHP encontrada/instalada.
// ── Laravel Base copy ─────────────────────────────────────────────────────────

/// Localiza o diretório `laravel-base` distribuído junto com o binário.
///
/// Busca em ordem:
///  1. Mesmo diretório do executável em execução
///  2. Diretório de trabalho atual
fn find_laravel_base() -> Option<std::path::PathBuf> {
    // 1. ao lado do binário
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let candidate = dir.join("laravel-base");
            if candidate.join("composer.json").exists() {
                return Some(candidate);
            }
        }
    }
    // 2. CWD (útil durante desenvolvimento)
    if let Ok(cwd) = std::env::current_dir() {
        let candidate = cwd.join("laravel-base");
        if candidate.join("composer.json").exists() {
            return Some(candidate);
        }
    }
    None
}

/// Copia recursivamente `src` para `dst`, ignorando `vendor/`, `.git/` e `.env`.
fn copy_dir_recursive(src: &std::path::Path, dst: &std::path::Path) -> Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let name = entry.file_name();
        let name_str = name.to_string_lossy();

        // Pula artefatos que não devem ser copiados para o projeto destino.
        if matches!(name_str.as_ref(), "vendor" | ".git" | ".env") {
            continue;
        }

        let src_path = entry.path();
        let dst_path = dst.join(&name);

        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            if let Some(parent) = dst_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

/// Copia o template `laravel-base/` para `dest`, inicializando um novo projeto
/// Laravel sem precisar do Composer. O usuário deve rodar `composer install`
/// manualmente após a geração.
pub fn copy_laravel_base(dest: &str) -> Result<()> {
    let src = find_laravel_base().ok_or_else(|| anyhow::anyhow!(
        "Diretório 'laravel-base' não encontrado.\n\
         Certifique-se de que ele está no mesmo diretório do binário 'caronte'."
    ))?;

    let dst = std::path::Path::new(dest);

    if dst.join("composer.json").exists() {
        println!(
            "  {} Projeto Laravel já existe em '{}' — pulando cópia do template.",
            style("ℹ").cyan(),
            style(dest).dim()
        );
        return Ok(());
    }

    println!(
        "  {} Copiando template Laravel para '{}'...",
        style("→").cyan(),
        style(dest).white().bold()
    );

    copy_dir_recursive(&src, dst)?;

    println!("  {} Template Laravel copiado.", style("✔").green().bold());
    println!();
    println!("  {} Próximos passos após a geração:", style("LEMBRETE:").yellow().bold());
    println!("    cd {}", dest);
    println!("    composer install");
    println!("    cp .env.example .env");
    println!("    php artisan key:generate");
    println!();

    Ok(())
}

pub fn verify_all() -> Result<String> {
    println!();
    println!(
        "  {}",
        style("[ deps ] Verificando dependências do ambiente...").cyan().bold()
    );
    println!();

    // PHP
    print!("  {} PHP 8.3 LTS...    ", style("▸").dim());
    let php_version = ensure_php()?;
    println!(
        "  {} PHP {} — OK",
        style("✔").green().bold(),
        style(&php_version).white().bold()
    );
    println!();

    // Composer
    print!("  {} Composer...       ", style("▸").dim());
    ensure_composer()?;
    println!("  {} Composer — OK", style("✔").green().bold());
    println!();

    // PostgreSQL
    print!("  {} PostgreSQL 16...  ", style("▸").dim());
    ensure_postgres()?;
    println!("  {} PostgreSQL — OK", style("✔").green().bold());
    println!();

    Ok(php_version)
}
