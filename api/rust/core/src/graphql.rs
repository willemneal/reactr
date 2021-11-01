use crate::errors::HostResult;
use crate::{env, ffi};

/// Retreives the result from the host and returns it
pub fn query(endpoint: &str, query: &str) -> HostResult<Vec<u8>> {
	let endpoint_size = endpoint.len() as i32;
	let query_size = query.len() as i32;

	let result_size = env::graphql_query(endpoint.as_ptr(), endpoint_size, query.as_ptr(), query_size);

	ffi::result(result_size)
}
