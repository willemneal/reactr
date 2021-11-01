pub mod query;

use crate::{env, errors::HostResult, ffi};

use query::{QueryArg, QueryType};

// insert executes the pre-loaded database query with the name <name>,
// and passes the arguments defined by <args>
//
// the return value is the inserted auto-increment ID from the query result, if any,
// formatted as JSON with the key `lastInsertID`
pub fn insert(name: &str, args: Vec<QueryArg>) -> HostResult<Vec<u8>> {
	args.iter().for_each(|arg| ffi::add_var(&arg.name, &arg.value));

	let result_size = env::db_exec(QueryType::INSERT.into(), name.as_ptr(), name.len() as i32);

	// retreive the result from the host and return it
	ffi::result(result_size)
}

// insert executes the pre-loaded database query with the name <name>,
// and passes the arguments defined by <args>
//
// the return value is the query result formatted as JSON, with each column name as a top-level key
pub fn select(name: &str, args: Vec<QueryArg>) -> HostResult<Vec<u8>> {
	args.iter().for_each(|arg| ffi::add_var(&arg.name, &arg.value));

	let result_size = env::db_exec(QueryType::SELECT.into(), name.as_ptr(), name.len() as i32);

	// retreive the result from the host and return it
	ffi::result(result_size)
}
