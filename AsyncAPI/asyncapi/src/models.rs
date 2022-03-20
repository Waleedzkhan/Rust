use serde::{Deserialize,Serialize};
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Customer
{
   pub Guid:String,
   pub FirstName:String,
   pub LastName:String,
   pub Email:String,
   pub Address:String, 
}