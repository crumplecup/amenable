//! Certificate-artifact writers for the built-in Rust standard-library
//! type registrations.

use std::{fs, io, path::Path, path::PathBuf};

use amenable_core::{Registry as _, Standard as _};

use crate::{RustStdStandard, RustStdType};

fn write_standard_certificate_artifact<T>(
    directory: &Path,
    registry: &mut crate::CertRegistry,
    file_stem: &str,
) -> io::Result<PathBuf>
where
    T: RustStdType,
{
    let certificate =
        RustStdStandard::<T>::new().certification(registry, std::any::type_name::<T>());
    let path = directory.join(format!("{file_stem}.provenance.txt"));

    fs::write(&path, certificate.to_string())?;

    Ok(path)
}

/// Emit provenance certificate artifacts for every built-in Rust standard type registration.
pub fn write_rust_std_certificate_artifacts(
    directory: impl AsRef<Path>,
) -> io::Result<Vec<PathBuf>> {
    let directory = directory.as_ref();
    let mut registry = crate::CertRegistry::new();
    let mut paths = Vec::new();

    fs::create_dir_all(directory)?;

    paths.push(write_standard_certificate_artifact::<bool>(
        directory,
        &mut registry,
        "bool",
    )?);
    paths.push(write_standard_certificate_artifact::<char>(
        directory,
        &mut registry,
        "char",
    )?);
    paths.push(write_standard_certificate_artifact::<i8>(
        directory,
        &mut registry,
        "i8",
    )?);
    paths.push(write_standard_certificate_artifact::<i16>(
        directory,
        &mut registry,
        "i16",
    )?);
    paths.push(write_standard_certificate_artifact::<i32>(
        directory,
        &mut registry,
        "i32",
    )?);
    paths.push(write_standard_certificate_artifact::<i64>(
        directory,
        &mut registry,
        "i64",
    )?);
    paths.push(write_standard_certificate_artifact::<i128>(
        directory,
        &mut registry,
        "i128",
    )?);
    paths.push(write_standard_certificate_artifact::<isize>(
        directory,
        &mut registry,
        "isize",
    )?);
    paths.push(write_standard_certificate_artifact::<u8>(
        directory,
        &mut registry,
        "u8",
    )?);
    paths.push(write_standard_certificate_artifact::<u16>(
        directory,
        &mut registry,
        "u16",
    )?);
    paths.push(write_standard_certificate_artifact::<u32>(
        directory,
        &mut registry,
        "u32",
    )?);
    paths.push(write_standard_certificate_artifact::<u64>(
        directory,
        &mut registry,
        "u64",
    )?);
    paths.push(write_standard_certificate_artifact::<u128>(
        directory,
        &mut registry,
        "u128",
    )?);
    paths.push(write_standard_certificate_artifact::<usize>(
        directory,
        &mut registry,
        "usize",
    )?);
    paths.push(write_standard_certificate_artifact::<f32>(
        directory,
        &mut registry,
        "f32",
    )?);
    paths.push(write_standard_certificate_artifact::<f64>(
        directory,
        &mut registry,
        "f64",
    )?);
    paths.push(write_standard_certificate_artifact::<String>(
        directory,
        &mut registry,
        "string",
    )?);

    fs::write(
        directory.join("registry-report.txt"),
        registry.report().to_string(),
    )?;

    Ok(paths)
}
