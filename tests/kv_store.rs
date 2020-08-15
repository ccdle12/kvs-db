use kvs::{KvStore, KvStoreError, KvsEngine, Result};
use tempfile::TempDir;
use walkdir::WalkDir;

// Should be able to set a key/value pair and retrieve it.
#[test]
fn get_stored_value() -> Result<()> {
    let temp_dir = TempDir::new().expect("unable to create temporary working directory");
    let mut store = KvStore::open(temp_dir.path())?;

    // TODO: to_owned may not be necessary as it clones the value.
    store.set("key1".into(), "value1".into())?;
    store.set("key2".into(), "value2".into())?;

    assert_eq!(store.get("key1".into())?, Some("value1".into()));
    assert_eq!(store.get("key2".into())?, Some("value2".into()));

    // Drop the store from memory and open the connection again. This will force
    // the DB to re-read the log file from disk and repopulate the in memory
    // offset map.
    drop(store);
    let store = KvStore::open(temp_dir.path())?;
    assert_eq!(store.get("key1".into())?, Some("value1".into()));
    assert_eq!(store.get("key2".into())?, Some("value2".into()));

    Ok(())
}

/// This test should overwrite the previous key value pair.
#[test]
fn overwrite_value() -> Result<()> {
    let temp_dir = TempDir::new().expect("unable to create temporary working directory");
    let mut store = KvStore::open(temp_dir.path())?;

    // Overwrite a key and expect the value to be changed on a get request.
    store.set("key1".into(), "value1".into())?;
    assert_eq!(store.get("key1".into())?, Some("value1".into()));

    store.set("key1".into(), "value2".into())?;
    assert_eq!(store.get("key1".into())?, Some("value2".into()));

    // Open from disk again and assert previously persisted values and overwrite
    // a key.
    drop(store);
    let mut store = KvStore::open(temp_dir.path())?;
    assert_eq!(store.get("key1".into())?, Some("value2".into()));

    store.set("key1".into(), "value3".into())?;
    assert_eq!(store.get("key1".into())?, Some("value3".into()));

    Ok(())
}

// Should get `None` when getting a non-existent key
#[test]
fn get_non_existent_value() -> Result<()> {
    let temp_dir = TempDir::new().expect("unable to create temporary working directory");
    let mut store = KvStore::open(temp_dir.path())?;

    store.set("key1".into(), "value1".into())?;
    assert!(store.get("key2".into()).is_err());

    // Open from disk again and check persistent data
    drop(store);
    let store = KvStore::open(temp_dir.path())?;
    assert!(store.get("key2".into()).is_err());

    Ok(())
}

#[test]
fn remove_non_existent_key() -> Result<()> {
    let temp_dir = TempDir::new().expect("unable to create temporary working directory");

    let mut store = KvStore::open(temp_dir.path())?;
    assert!(store.remove("key1".into()).is_err());

    Ok(())
}

#[test]
fn remove_key() -> Result<()> {
    let temp_dir = TempDir::new().expect("unable to create temporary working directory");

    let mut store = KvStore::open(temp_dir.path())?;
    store.set("key1".into(), "value1".into())?;

    assert!(store.remove("key1".into()).is_ok());
    assert!(store.get("key1".into()).is_err());

    // Drop the connection, reopen and check that reading from the log file
    // on open does not crash.
    drop(store);
    let store = KvStore::open(temp_dir.path())?;

    // Getting key1 should return a KeyNotFoundError.
    assert!(store.get("key1".into()).is_err());

    Ok(())
}

// // Insert data until total size of the directory decreases.
// // Test data correctness after compaction.
// #[test]
// fn compaction() -> Result<()> {
//     let temp_dir = TempDir::new().expect("unable to create temporary working directory");
//     let mut store = KvStore::open(temp_dir.path())?;

//     let dir_size = || {
//         let entries = WalkDir::new(temp_dir.path()).into_iter();
//         let len: walkdir::Result<u64> = entries
//             .map(|res| {
//                 res.and_then(|entry| entry.metadata())
//                     .map(|metadata| metadata.len())
//             })
//             .sum();
//         len.expect("fail to get directory size")
//     };

//     let mut current_size = dir_size();
//     for iter in 0..1000 {
//         for key_id in 0..1000 {
//             let key = format!("key{}", key_id);
//             let value = format!("{}", iter);
//             store.set(key, value)?;
//         }

//         let new_size = dir_size();
//         if new_size > current_size {
//             current_size = new_size;
//             continue;
//         }
//         // Compaction triggered

//         drop(store);
//         // reopen and check content
//         let mut store = KvStore::open(temp_dir.path())?;
//         for key_id in 0..1000 {
//             let key = format!("key{}", key_id);
//             assert_eq!(store.get(key)?, Some(format!("{}", iter)));
//         }
//         return Ok(());
//     }

//     panic!("No compaction detected");
// }
