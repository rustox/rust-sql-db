extern crate postgres;

use postgres::Connection;
use postgres::TlsMode;

fn main() {

    let host="10.10.10.7";
    let port="5432";
	let db  ="rustox";
	let user="rustox";
	let pass="rustox";

	let mut dburi = String::with_capacity(10);
	
	dburi.push_str("postgres");
	dburi.push_str("://");
	dburi.push_str(user);
	dburi.push_str(":");
	dburi.push_str(pass);
	dburi.push_str("@");
	dburi.push_str(host);
	dburi.push_str(":");
	dburi.push_str(port);
	dburi.push_str("/");
	dburi.push_str(db);

	println! ("dburi is {}", dburi);

    let con = Connection::connect(dburi, TlsMode::None).unwrap();

	assert! (con.finish().is_ok());
}

