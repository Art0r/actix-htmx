use diesel::{PgConnection, RunQueryDsl};
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use crate::errors::user::UserError;
use crate::forms::pet::CreatePetsForm;
use crate::models::pet::{NewPet, Pet};
use crate::models::user::User;
use crate::schema::schema::pets::dsl::*;
use diesel::prelude::*;
use diesel::associations::HasTable;
use crate::schema::schema::users::dsl::users;

pub struct PetsService {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PetsService {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        PetsService { pool }
    }

    fn get_conn(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.pool.get().expect("Couldn't get DB connection")
    }
    
    pub fn create_pet(&self, create_pets_form: CreatePetsForm) -> Result<Pet, UserError> {
        let mut conn = self.get_conn();
        
        let new_pet = NewPet {
            name: create_pets_form.name,
            description: create_pets_form.description,
            tutor_id: create_pets_form.tutor_id,
        };

        diesel::insert_into(pets::table())
            .values(&new_pet)
            .returning(Pet::as_returning())
            .get_result(&mut conn)
            .map_err(UserError::from)
    }
    
    pub fn get_all_pets(&self) -> QueryResult<Vec<Pet>> {
        let mut conn = self.get_conn();
        pets.load::<Pet>(&mut conn)
    }
    
    pub fn get_one_pet_by_id(&self, pet_id: i32) -> QueryResult<Option<Pet>> {
        let mut conn = self.get_conn();
        pets.filter(id.eq(pet_id))
            .first(&mut conn)
            .optional()
    }

    pub fn delete_pet(&self, pet_id: i32) -> Result<Pet, UserError> {
        let mut conn = self.get_conn();
        
        diesel::delete(pets::table())
            .filter(id.eq(pet_id))
            .returning(Pet::as_returning())
            .get_result(&mut conn)
            .map_err(UserError::from)
    }
}