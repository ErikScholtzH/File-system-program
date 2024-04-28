use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::fs::File;
use std::io::Error;
use std::path::Path;
use sysinfo;

#[derive(Debug)]
pub enum FSError {
    FileSystemNotFound,
    FileSystemOperationError,
    FileSystemAllocationError,
}

pub struct FSEntry<'a> { // TODO: figure who how this lifetime stuff works
    name: String,
    f_type: String,
    path: &'a Path,
    size: u64,
}

impl <'a> FSEntry <'a> { // TODO: FIXME there needs ot be error cases handled 
    pub fn new(name: String, f_type: String, path: &'a Path, size: u64) -> Result<Self, FSError>{
        let entry = FSEntry{
            name,
            f_type,
            path,
            size,
        };
        Ok(entry)
    }
}

pub struct Disk {
    name: String,
    disk_type: String,
    mount: String,
    fs_type: String,
    full_size: u64,
    curr_size: u64,
    is_removable: bool,
}


impl Disk {
    fn print_files_in_directory(dir_path: &Path) -> Result<(), std::io::Error> {
        if let Ok(entries) = std::fs::read_dir(dir_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let metadata = entry.metadata()?;
                    let file_name = entry.file_name();
                    if metadata.is_file() {
                        let file_size = metadata.len(); // Size in bytes
                        println!("File: {:?}, Size: {} bytes", file_name, file_size);
                        
                    } else if metadata.is_dir() {
                        println!("Directory: {:?}", file_name);
                    }
                }
            }
        }
        Ok(())
    }
}

impl Serialize for Disk {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Disk", 1)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("disk type", &self.disk_type)?;
        state.serialize_field("mount point", &self.mount)?;
        state.serialize_field("fs type", &self.fs_type)?;
        state.serialize_field("external", &self.is_removable)?;
        state.serialize_field("size", &self.full_size)?;
        state.serialize_field("avaliable", &self.curr_size)?;
        state.end()
    }
}

pub struct FileSystem {
    disks: Vec<Disk>,
}

impl FileSystem {
    pub fn new() -> Result<Self, FSError> {
        let found_disks = sysinfo::Disks::new_with_refreshed_list();
        let mut disks: Vec<Disk> = Vec::new();
        for disk in found_disks.list() {
            let name = disk.name().to_string_lossy().into_owned();
            let mount = disk.mount_point().to_string_lossy().into_owned();
            let disk_type = disk.kind().to_string();
            let full_size = disk.total_space();
            let curr_size = disk.available_space();
            let is_removable = disk.is_removable();
            let fs_type = disk.file_system().to_string_lossy().into_owned();
            let fs_disk = Disk {
                name,
                disk_type,
                mount,
                fs_type,
                full_size,
                curr_size,
                is_removable,
            };
            disks.push(fs_disk);
            match Disk::print_files_in_directory(disk.mount_point()) {
                Ok(()) => {
                    println!("Successfully printed files in directory {}", disk.mount_point().to_string_lossy().into_owned());
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                }
            }
            
        }
        Ok(FileSystem { disks })
    }

    pub fn get_disks(&self) -> &Vec<Disk> {
        &self.disks
    }

    fn create_file(path: &Path) -> Result<(), Error> {
        // TODO: understand what fs.rs is
        match File::create(path) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
}
