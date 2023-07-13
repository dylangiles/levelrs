use core::slice;
use std::{
    error::Error,
    ffi::{c_void, CStr, CString},
    ptr,
    str::{from_utf8, Utf8Error},
};

use leveldb::{
    leveldb_free, leveldb_get, leveldb_options_t, leveldb_put, leveldb_readoptions_create,
    leveldb_readoptions_set_fill_cache, leveldb_readoptions_set_snapshot,
    leveldb_readoptions_set_verify_checksums, leveldb_readoptions_t, leveldb_t,
    leveldb_writeoptions_create, leveldb_writeoptions_set_sync, leveldb_writeoptions_t,
    SetReadOptionFunction,
};

use crate::leveldb::{
    leveldb_close, leveldb_open, leveldb_options_create, leveldb_options_set_create_if_missing,
};

mod leveldb;

#[derive(Debug)]

pub struct Database {
    options: *mut leveldb_options_t,
    leveldb: *mut leveldb_t,
    read_options: *const leveldb_readoptions_t,
    write_options: *const leveldb_writeoptions_t,
}

#[derive(Debug)]
pub enum LevelDBError {
    Raw(String),
    Utf8(Utf8Error),
}

impl LevelDBError {
    pub fn new_from_char(data: *const i8) -> Self {
        let err_string = from_utf8(unsafe { CStr::from_ptr(data) }.to_bytes())
            .unwrap()
            .to_string();
        unsafe { leveldb_free(data as *mut c_void) };
        Self::Raw(err_string)
    }
}

impl Error for LevelDBError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

impl std::fmt::Display for LevelDBError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LevelDBError::Raw(e) => write!(f, "LevelDB error: {}", e),
            LevelDBError::Utf8(ue) => write!(f, "UTF-8 error: {ue}"),
        }
    }
}

impl Database {
    pub fn new() -> Self {
        Self {
            options: unsafe { leveldb_options_create() },
            leveldb: ptr::null_mut(),
            read_options: unsafe { leveldb_readoptions_create() },
            write_options: unsafe { leveldb_writeoptions_create() },
        }
    }

    pub fn create_if_missing(self) -> Self {
        unsafe { leveldb_options_set_create_if_missing(self.options, 1) }
        self
    }

    pub fn write_sync(self, value: bool) -> Self {
        unsafe { leveldb_writeoptions_set_sync(self.write_options, if value { 1 } else { 0 }) }
        self
    }

    pub fn verify_checksums(self, value: bool) -> Self {
        self.set_read_option(leveldb_readoptions_set_verify_checksums, value)
    }

    pub fn fill_cache(self, value: bool) -> Self {
        self.set_read_option(leveldb_readoptions_set_fill_cache, value)
    }

    pub fn snapshot(self, value: bool) -> Self {
        self.set_read_option(leveldb_readoptions_set_snapshot, value)
    }

    fn set_read_option(self, function: SetReadOptionFunction, value: bool) -> Self {
        unsafe { function(self.read_options, if value { 1 } else { 0 }) }
        self
    }

    pub fn open(mut self, name: &str) -> Result<Self, LevelDBError> {
        let mut err = ptr::null_mut();
        let db_name = CString::new(name).unwrap();

        self.leveldb = unsafe { leveldb_open(self.options, db_name.as_ptr(), &mut err) };
        if err == ptr::null_mut() {
            Ok(self)
        } else {
            Err(LevelDBError::new_from_char(err))
        }
    }

    pub fn put(&self, key: &str, value: &str) -> Result<(), LevelDBError> {
        let mut err = ptr::null_mut();
        unsafe {
            leveldb_put(
                self.leveldb,
                self.write_options,
                key.as_ptr(),
                key.len(),
                value.as_ptr(),
                value.len(),
                &mut err,
            )
        }

        if err == ptr::null_mut() {
            Ok(())
        } else {
            Err(LevelDBError::new_from_char(err))
        }
    }

    pub fn get_raw(&self, key: &str) -> Result<(*const u8, usize), LevelDBError> {
        let mut err = ptr::null_mut();
        let mut vallen = 0usize;
        let result = unsafe {
            leveldb_get(
                self.leveldb,
                self.read_options,
                key.as_ptr(),
                key.len(),
                &mut vallen,
                &mut err,
            )
        };

        println!("{:#?}", result);
        if err == ptr::null_mut() {
            Ok((result, vallen))
        } else {
            Err(LevelDBError::new_from_char(err))
        }
    }

    pub fn get_string(&self, key: &str) -> Result<String, LevelDBError> {
        let (result, length) = self.get_raw(key)?;
        let slice = unsafe { slice::from_raw_parts(result, length) };
        match from_utf8(slice) {
            Ok(u) => Ok(u.to_string()),
            Err(e) => Err(LevelDBError::Utf8(e)),
        }
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        if self.leveldb != ptr::null_mut() {
            unsafe { leveldb_close(self.leveldb) }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Database;

    #[test]
    fn put_ok() {
        let db = Database::new()
            .create_if_missing()
            .open("test/db")
            .unwrap()
            .write_sync(true);

        match db.put("hello", "world") {
            Ok(_) => {}
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn get_returns_correct_string() {
        const TEST_KEY: &str = "hello";
        const EXPECTED_VALUE: &str = "world";

        let db = Database::new()
            .create_if_missing()
            .open("test/db")
            .unwrap()
            .write_sync(true);

        match db.put(TEST_KEY, EXPECTED_VALUE) {
            Ok(_) => match db.get_string(TEST_KEY) {
                Ok(v) => assert_eq!(v, EXPECTED_VALUE),
                Err(e) => panic!("{}", e),
            },
            Err(e) => panic!("{}", e),
        }
    }
}
