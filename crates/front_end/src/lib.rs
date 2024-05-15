use data_store::database_api_call;

pub fn app_api_call(data: u32) -> u32 {
    println!("API Running!");
    let db_result = database_api_call(data);
    println!("API shutting down");
    db_result + 1
}
