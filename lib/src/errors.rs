#[cfg(feature = "rocksdb-datastore")]
use bincode::Error as BincodeError;
#[cfg(feature = "rocksdb-datastore")]
use rocksdb::Error as RocksDbError;
use serde_json::Error as JsonError;

error_chain!{
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        Json(JsonError);
        RocksDb(RocksDbError) #[cfg(feature = "rocksdb-datastore")];
        Bincode(BincodeError) #[cfg(feature = "rocksdb-datastore")];
    }

    errors {
        UuidConflict {
            description("UUID already taken")
            display("The autogenerated UUID was expected to be unique, but was already taken")
        }
    }
}

error_chain! {
    types {
        ValidationError, ValidationErrorKind, ValidationResultExt, ValidationResult;
    }
}
