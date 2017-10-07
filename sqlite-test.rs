extern crate rusqlite;
extern crate time;

use time::Timespec;
use rusqlite::Connection;

#[derive(Debug)]
struct Member {
	id: 	i32,
	name:	String,
	timec:	Timespec,
	data:	Option<Vec<u8>>
}


fn main() {

	let con = Connection::open_in_memory().unwrap();

	con.execute(
	"CREATE TABLE Member(
		id		INTEGER PRIMARY KEY,
		name	TEXT NOT NULL,
		timec	TEXT NOT NULL,
		data	BLOB
	)",
    &[]).unwrap();


	let m1 = Member {
		id:		0,
		name:	"Ronak".to_string(),
		timec:	time::get_time(),
		data:	None
	};


	con.execute(
	"INSERT INTO Member (name, timec, data)
	VALUES (?1, ?2, ?3)",
	&[ &m1.name, &m1.timec, &m1.data]
	).unwrap();


	let mut stmt = con.prepare(
	"SELECT id, name, timec, data FROM Member"
	).unwrap();

	let member_iterator = stmt.query_map( &[], |row| {
		Member {
			id:		row.get(0),
			name:	row.get(1),
			timec:	row.get(2),
			data:	row.get(3),
		}
	}).unwrap();	


	for member in member_iterator {
		println! ("member {:?}", member.unwrap());
	}
}
