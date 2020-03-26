
extern crate oracle;

use oracle::Connection;

#[allow(dead_code)]
pub fn oracle_usage() {

    println!("About to connect...");

    // Connect to a database.
    let conn = Connection::connect("GSTSVCAPP", "gstsvcapp", "//localhost/XE", &[]).unwrap();

    let mut stmt = conn
        .prepare(
            "select PGM_DESC from DGST.LYTY_PGM",
            &[],
        )
        .unwrap();
    let rows = stmt.query(&[]).unwrap();

// stmt.define("HIREDATE", OracleType::Varchar2(60)).unwrap();

    for info in rows.column_info() {
        println!(
            " {:-30} {:-8} {}",
            info.name(),
            if info.nullable() { "" } else { "NOT NULL" },
            info.oracle_type()
        );
    }
    println!("");

    for row_result in rows {
        let row = row_result.unwrap();
        let program_description: String = row.get(0).unwrap(); // index by 0-based position
        // let ename: String = row.get("ENAME").unwrap(); // index by case-sensitive string
        // let job: String = row.get(2).unwrap();
        // let mgr: Option<i32> = row.get(3).unwrap(); // nullable column must be get as Option<...> to avoid panic.
        // let hiredate: Timestamp = row.get(4).unwrap();
        // let sal: f64 = row.get(5).unwrap();
        // let comm: Option<f64> = row.get(6).unwrap();
        // let deptno: Option<i32> = row.get(7).unwrap();

        println!(
            "{}", // "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            &program_description
            // ename,
            // job,
            // mgr.map_or("".to_string(), |v| v.to_string()), // empty string if None, otherwise content of Some(x).
            // hiredate,
            // sal,
            // comm.map_or("".to_string(), |v| v.to_string()),
            // deptno.map_or("".to_string(), |v| v.to_string())
        );
    }
}