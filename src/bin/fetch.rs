// extern crate qc_bufpo;
// 
// use qc_bufpo::frame::*;
// 
// fn main() {
//     let mut bupo = QcBupo::new("hello.db");
// 
//     let Ok(pg) = bupo.fetch_page(4) else {
//         println!("fetch error!");
//         return;
//     };
// 
//     let mpg = pg.borrow_mut();
// 
//     bupo.report();
// 
// }
