use crate::{ffi, env, errors::HostResult};



pub fn set(key: &str, val: Vec<u8>, ttl: i32) {
	let val_slice = val.as_slice();
	let val_ptr = val_slice.as_ptr();
		env::cache_set(
			key.as_ptr(),
			key.len() as i32,
			val_ptr,
			val.len() as i32,
			ttl,
		);
}

/// Executes the request via FFI
///
/// Then retreives the result from the host and returns it
pub fn get(key: &str) -> HostResult<Vec<u8>> {
	let result_size = env::cache_get(key.as_ptr(), key.len() as i32);

	ffi::result(result_size)
}
