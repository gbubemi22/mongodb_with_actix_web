use chrono::Utc;
use dotenv::var;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use mongodb::bson::DateTime as BsonDateTime;
use mongodb::bson::{doc, from_document, oid::ObjectId};
use mongodb::results::{InsertOneResult, UpdateResult};
use mongodb::{Client, Collection};
use std::{error::Error, str::FromStr};

use crate::models::booking_model::FullBooking;

#[derive(Debug, Serialize, Deserialize)] 
pub struct Booking;

#[derive(Debug, Serialize, Deserialize)] 
pub struct Dog;

#[derive(Debug, Serialize, Deserialize)] 
pub struct Owner;

pub struct Database {
    booking: Collection<Booking>,
    dog: Collection<Dog>,
    owner: Collection<Owner>,
}

impl Database {
    pub async fn init() -> Result<Self, Box<dyn Error>> {
        let uri = var("MONGO_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
        let client = Client::with_uri_str(&uri).await?;
        let db = client.database("dog_walking");

        let booking: Collection<Booking> = db.collection("booking");
        let dog: Collection<Dog> = db.collection("dog");
        let owner: Collection<Owner> = db.collection("owner");

        Ok(Database {
            booking,
            dog,
            owner,
        })
    }

    pub async fn create_owner(&self, owner: Owner) -> Result<InsertOneResult, Box<dyn Error>> {
        let result = self.owner.insert_one(owner).await?;
        Ok(result)
    }

    pub async fn create_dog(&self, dog: Dog) -> Result<InsertOneResult, Box<dyn Error>> {
        let result = self.dog.insert_one(dog).await?;
        Ok(result)
    }

    pub async fn create_booking(
        &self,
        booking: Booking,
    ) -> Result<InsertOneResult, Box<dyn Error>> {
        let result = self.booking.insert_one(booking).await?;
        Ok(result)
    }

    pub async fn cancel_booking(&self, booking_id: &str) -> Result<UpdateResult, Box<dyn Error>> {
        let result = self
            .booking
            .update_one(
                doc! {
                    "_id": ObjectId::from_str(booking_id)?,
                },
                doc! {
                    "$set": { "cancelled": true }
                },
                
            )
            .await?;
        Ok(result)
    }

    pub async fn get_bookings(&self) -> Result<Vec<FullBooking>, Box<dyn Error>> {
        let now = Utc::now();
        let now_bson = BsonDateTime::from_millis(now.timestamp_millis());
        let mut results = self
            .booking
            .aggregate(vec![
                doc! {
                    "$match": {
                        "cancelled": false,
                        "start_time": { "$gte": now_bson }
                    }
                },
                doc! {
                    "$lookup": {
                        "from": "owner",
                        "localField": "owner",
                        "foreignField": "_id",
                        "as": "owner"
                    }
                },
                doc! {
                    "$unwind": "$owner"
                },
                doc! {
                    "$lookup": {
                        "from": "dog",
                        "localField": "owner._id",
                        "foreignField": "owner",
                        "as": "dogs"
                    }
                },
            ]) // Provide an aggregate option if needed
            .await?;

        let mut bookings: Vec<FullBooking> = Vec::new();

        while let Some(result) = results.next().await {
            match result {
                Ok(doc) => {
                    let booking: FullBooking = from_document(doc)?;
                    bookings.push(booking);
                }
                Err(err) => return Err(Box::new(err)),
            }
        }
        Ok(bookings)
    }
}
