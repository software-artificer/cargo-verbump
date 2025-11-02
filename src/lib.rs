use anyhow::Context;
use std::{fs, path};

#[derive(clap::Parser)]
#[command(version, about)]
pub struct Args {
    #[arg(short, long, default_value = "Cargo.toml")]
    path: path::PathBuf,
    #[arg()]
    new_version: semver::Version,
}

pub fn update_version(args: Args) -> anyhow::Result<()> {
    let cargo = fs::read_to_string(&args.path)
        .with_context(|| format!("Failed to read the `{}` file", args.path.display()))?;

    let mut doc: toml_edit::DocumentMut = cargo
        .parse()
        .with_context(|| format!("Failed to parse the `{}` file", args.path.display()))?;

    doc["package"]["version"] = toml_edit::value(args.new_version.to_string());

    fs::write(&args.path, doc.to_string())
        .with_context(|| format!("Failed to write the `{}` file", args.path.display()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn read_fail() {
        let temp = tempfile::tempdir().expect("Failed to create a temporary test directory");

        let path = temp.path().join("Cargo.toml");

        let args = super::Args {
            path: path.clone(),
            new_version: semver::Version::new(0, 1, 0),
        };

        let result = super::update_version(args)
            .expect_err("Expected update_version() to fail due to missing file, but it succeeded");

        assert_eq!(
            format!("Failed to read the `{}` file", path.display()),
            result.to_string(),
            "Expected update_version() to fail due to file read operation",
        );
    }

    #[test]
    fn parse_fail() {
        let temp = tempfile::tempdir().expect("Failed to create a temporary test directory");

        let path = temp.path().join("Cargo.toml");

        fs::write(&path, "this is not a good TOML contents btw")
            .expect("Failed to create a test Cargo.toml file");

        let args = super::Args {
            path: path.clone(),
            new_version: semver::Version::new(0, 1, 0),
        };

        let result = super::update_version(args)
            .expect_err("Expected update_version() to fail due to parsing error, but it succeeded");

        assert_eq!(
            format!("Failed to parse the `{}` file", path.display()),
            result.to_string(),
            "Expected update_version() to fail due to file read operation",
        );
    }

    #[test]
    fn write_fail() {
        let temp = tempfile::tempdir().expect("Failed to create a temporary test directory");

        let path = temp.path().join("Cargo.toml");

        fs::write(
            &path,
            r#"[package]
name = "test"
version = "0.0.0"
"#,
        )
        .expect("Failed to create a test Cargo.toml file");

        let metadata =
            fs::metadata(&path).expect("Failed to read the test Cargo.toml file metadata");

        let mut perms = metadata.permissions();

        perms.set_readonly(true);

        fs::set_permissions(&path, perms)
            .expect("Failed to make the test Cargo.toml file read-only");

        let args = super::Args {
            path: path.clone(),
            new_version: semver::Version::new(0, 1, 0),
        };

        let result = super::update_version(args)
            .expect_err("Expected update_version() to fail due to parsing error, but it succeeded");

        assert_eq!(
            format!("Failed to write the `{}` file", path.display()),
            result.to_string(),
            "Expected update_version() to fail due to file read operation",
        );
    }

    #[test]
    fn success() {
        let temp = tempfile::tempdir().expect("Failed to create a temporary test directory");

        let path = temp.path().join("Cargo.toml");

        fs::write(
            &path,
            r#"[package]
name = "test"
version = "0.0.0"
"#,
        )
        .expect("Failed to create a test Cargo.toml file");

        let args = super::Args {
            path: path.clone(),
            new_version: semver::Version::new(0, 1, 0),
        };

        super::update_version(args)
            .expect("Expected update_version() to fail due to parsing error, but it succeeded");

        let result = fs::read_to_string(&path)
            .expect("Failed to read the resulting Cargo.toml file to string");

        assert_eq!(
            r#"[package]
name = "test"
version = "0.1.0"
"#,
            result,
            "The resulting Cargo.toml file is incorrect",
        );
    }
}
