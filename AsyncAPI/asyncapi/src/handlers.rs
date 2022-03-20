use std::convert::Infallible;
use warp::{self, http::StatusCode};
use crate::models::Customer;
use crate::db::Db;

pub async fn list_customers(db: Db) -> Result<impl warp::Reply, Infallible>{
    let customers = db.lock().await;
    let customers: Vec<Customer> = customers.clone();
    Ok(warp::reply::json(&customers))

 }

 pub async fn create_customer(
     new_customer: Customer,
     db: Db,
)-> Result<impl warp::Reply, Infallible>{
   let mut customers = db.lock().await;

   for customer in customers.iter(){
       if customer.Guid == new_customer.Guid{
           return Ok(StatusCode::BAD_REQUEST);
       }
   }
   customers.push(new_customer);
   Ok(StatusCode::CREATED)

}

pub async fn get_customer(guid: String, db: Db) -> Result<Box<dyn warp::Reply>, Infallible> {
    let customers = db.lock().await;

    for customer in customers.iter(){
        if customer.Guid == guid {
            return Ok(Box::new(warp::reply::json(&customer)));
        }
    }
    Ok(Box::new(StatusCode::NOT_FOUND))
}

pub async fn update_customer(guid:String, update_customer: Customer, db: Db,)-> Result<impl warp::Reply, Infallible>
{
  let mut customers = db.lock().await;

  for customer in customers.iter_mut(){
      if customer.Guid == guid{
          *customer = update_customer;
          return Ok(StatusCode::OK);
      }
  }
  Ok(StatusCode::NOT_FOUND)
}

pub async fn delete_customer(guid:String, db:Db)-> Result<impl warp::Reply, Infallible>
{
    let mut customers = db.lock().await;
    let customer_count = customers.len();
    customers.retain(|customer|customer.Guid != guid);
    let deleted = customers.len() != customer_count;
    if deleted{
        Ok(StatusCode::NO_CONTENT)
    }else{
        Ok(StatusCode::NOT_FOUND)
    }

}