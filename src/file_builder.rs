use super::{track_io, Error, Result};
#[cfg(unix)]
use libc;
use std::fs::{self, File};
use std::io;
use std::path::Path;

pub struct FileBuilder {
    direct_io: bool,
}

impl Default for FileBuilder {
    fn default() -> Self {
        FileBuilder { direct_io: true }
    }
}

impl FileBuilder {
    pub fn new() -> Self {
        FileBuilder::default()
    }

    #[cfg(target_os = "linux")]
    fn open_options(&self) -> fs::OpenOptions {
        use std::os::unix::fs::OpenOptionsExt;
        let mut options = fs::OpenOptions::new();
        options.read(true).write(true).create(false);

        if self.direct_io {
            options.custom_flags(libc::O_DIRECT);
        }
        options
    }
    #[cfg(not(target_os = "linux"))]
    fn open_options(&self) -> fs::OpenOptions {
        let mut options = fs::OpenOptions::new();
        options.read(true).write(true).create(false);
        options
    }

    #[cfg(target_os = "macos")]
    fn set_fnocache_if_flag_is_on(&self, file: &File) -> Result<()> {
        use std::os::unix::io::AsRawFd;

        if self.direct_io {
            if unsafe { libc::fcntl(file.as_raw_fd(), libc::F_NOCACHE, 1) } != 0 {
                track_io!(Err(io::Error::last_os_error()))
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }
    #[cfg(not(target_os = "macos"))]
    fn set_fnocache_if_flag_is_on(&self, _file: &File) -> Result<()> {
        Ok(())
    }

    pub fn direct_io(&mut self, enabled: bool) -> &mut Self {
        self.direct_io = enabled;
        self
    }

    #[cfg(target_os = "linux")]
    fn file_open_with_error_info<P: AsRef<Path>>(
        &self,
        do_create: bool, // This denoes whether or not the next argument `options` allows file creating.
        options: &fs::OpenOptions,
        filepath: &P,
    ) -> Result<File> {
        use std::os::unix::fs::OpenOptionsExt;

        let open_result = track_io!(options.open(&filepath));

        // If we succeed on opening the file `filepath`, we return it.
        if open_result.is_ok() {
            return open_result;
        }

        // Otherwise, we check why opening the file `filepath` failed.
        // First, we check the existence of the file `filepath`.
        if !std::path::Path::new(filepath.as_ref()).exists() {
            if do_create {
                // failed to file open
                return track!(
                    open_result,
                    "We failed to create the file {:?}.",
                    filepath.as_ref()
                );
            } else {
                // `do_create == false` means to open an existing file;
                // however, now the file `filepath` does not exist.
                return track!(
                    open_result,
                    "The file {:?} does not exist and failed to open it.",
                    filepath.as_ref()
                );
            }
        }

        // Next, we check if the file `filepath` can be opened without `O_DIRECT` option.
        let mut options = fs::OpenOptions::new();
        options.read(true).write(true).create(false);

        let file = track_io!(options.open(&filepath));
        if file.is_err() {
            return track!(file, "We cannot open the file {:?}.", filepath.as_ref());
        }

        // Finally, we check if the file `filepath` can be opened with `O_DIRECT` option.
        if self.direct_io {
            options.custom_flags(libc::O_DIRECT);
            let file = track_io!(options.open(&filepath));
            if file.is_err() {
                return track!(
                    file,
                    "We cannot open the file {:?} with O_DIRECT.",
                    filepath.as_ref()
                );
            }
        }

        // Strange case; so, we return the originanl error information.
        open_result
    }

    #[cfg(not(target_os = "linux"))]
    fn file_open_with_error_info<P: AsRef<Path>>(
        &self,
        _do_create: bool,
        options: &fs::OpenOptions,
        filepath: &P,
    ) -> Result<File> {
        track_io!(options.open(&filepath))
    }

    pub fn create_if_absent<P: AsRef<Path>>(&mut self, filepath: P) -> Result<(File, bool)> {
        create_parent_directories(&filepath)?;
        let mut options = self.open_options();
        // OpenOptions::createはファイルが既に存在する場合はそれを開き
        // 存在しない場合は作成する
        options.create(true);
        let file = track!(self.file_open_with_error_info(true, &options, &filepath))?;

        // metadataのファイルサイズの非ゼロ検査で
        // 新規作成されたファイルかどうかを判断する
        let metadata = track_io!(fs::metadata(&filepath))?;
        if metadata.len() == 0 {
            // ファイルが新しく作成された
            self.initialize(file).map(|s| (s, true))
        } else {
            self.initialize(file).map(|s| (s, false))
        }
    }

    pub fn create<P: AsRef<Path>>(&mut self, filepath: P) -> Result<File> {
        create_parent_directories(&filepath)?;
        let mut options = self.open_options();
        options.create_new(true);
        let file = self.file_open_with_error_info(true, &options, &filepath)?;
        self.initialize(file)
    }

    pub fn open<P: AsRef<Path>>(&mut self, filepath: P) -> Result<File> {
        let options = self.open_options();
        let file = self.file_open_with_error_info(false, &options, &filepath)?;
        self.initialize(file)
    }

    fn initialize(&self, file: File) -> Result<File> {
        track!(self.set_fnocache_if_flag_is_on(&file))?;
        Ok(file)
    }
}

/// 親ディレクトリの作成が必要な場合は作成する。
fn create_parent_directories<P: AsRef<Path>>(filepath: P) -> Result<()> {
    if let Some(dir) = filepath.as_ref().parent() {
        track_io!(fs::create_dir_all(dir))?;
    }
    Ok(())
}
