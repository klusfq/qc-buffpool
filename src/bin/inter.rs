// extern crate qc_bufpo;
// 
// use qc_bufpo::page::Pager;
// 
// struct QcDB {
//     pub target: Pager,
// }
// 
// impl QcDB {
//     pub fn new() -> Self {
//         return QcDB {
//             target: Pager::new(),
//         };
//     }
// 
//     pub fn insert(&mut self, k: i32, v: &str) {
//     }
// 
//     pub fn query(&self, k: i32) -> Option<String> {
//         return Some("".to_string());
//     }
// 
// }
// 
// fn main() {
//     let mut qc = QcDB::new();
// 
//     qc.insert(5, "hello");
//     qc.insert(2, "fuqx");
//     qc.insert(4, "qqccc");
// 
//     let result = qc.query(2);
// 
//     println!("result: {:?}", result);
// }
