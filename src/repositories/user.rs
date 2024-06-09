
use entity::user;
use migration::Expr;
use sea_orm::DbErr;
use sea_orm::DeleteResult;
use sea_orm::QueryFilter;
// use sea_orm::ConnectionTrait;
use sea_orm::QueryOrder;
use uuid;
use chrono;

use entity::user::Entity as User;
use entity::user::Model;
use entity::user::ActiveModel;
use sea_orm::ActiveValue::Set;
use sea_orm::{DatabaseConnection, EntityTrait, InsertResult};


pub trait Repository<'a, Entity> where Entity: EntityTrait {
    fn new(connection: &'a DatabaseConnection) -> Self;
    async fn find_by_uuid(&self, id: String) -> Option<Entity::Model>;
    async fn get_many(&self) -> Vec<Model>;
    async fn delete_by_uuid(&self, uuid: String) -> Result<DeleteResult, DbErr>;
}


pub struct UserRepository<'a> {
    connection: &'a DatabaseConnection
}
impl<'a> Repository<'a, User> for UserRepository<'a> {
    fn new(connection: &'a DatabaseConnection) -> Self {
        UserRepository{connection}
    }

    async fn find_by_uuid(&self, uuid: String) -> Option<Model> {

        let user: Option<Model> = User::find_by_id(uuid)
            .one(self.connection).await.unwrap();

        return user
    }

    async fn get_many(&self) -> Vec<Model> {
        let user: Vec<Model> = User::find()
            .order_by_asc(entity::user::Column::CreatedAt).all(self.connection)
            .await.unwrap();

        return user
    }

    async fn delete_by_uuid(&self, uuid: String) -> Result<DeleteResult, sea_orm::DbErr> {
        let uuid_col: Expr = Expr::col(user::Column::Uuid);

        let result: DeleteResult = User::delete_many()
            .filter(uuid_col.eq(uuid)).exec(self.connection).await?;

        Ok(result)
    }


}

impl<'a> UserRepository<'a> {
    
    #[allow(unused)]
    pub async fn update_user(
        &self,
        uuid: String,
        username: Option<String>,
        password_hash: Option<String>,
        name: Option<String>,
        email: Option<String>
    ) -> Result<Model, DbErr> {

        let uuid_col: Expr = Expr::col(user::Column::Uuid);
        let user_opt = User::find()
            .filter(uuid_col.eq(uuid.clone()))
            .one(self.connection)
            .await?;

        if let Some(user) = user_opt {

            let mut user: ActiveModel = user.into();

            if let Some(username) = username {
                user.username = Set(username);
            }
            if let Some(password_hash) = password_hash {
                user.password_hash = Set(password_hash);
            }
            if let Some(name) = name {
                user.name = Set(name);
            }
            if let Some(email) = email {
                user.email = Set(email);
            }

            let updated_user: Model = User::update(user)
            .exec(self.connection)
            .await?;

            Ok(updated_user)
        } else {
            Err(DbErr::RecordNotFound(format!("User {} not found", uuid)))
        }
    }

    #[allow(unused)]
    pub async fn create_user(
        &self,
        username: String,
        password_hash: String,
        name: String,
        email: String
    ) -> Result<InsertResult<ActiveModel>, sea_orm::DbErr>{

        let u: String = uuid::Uuid::new_v4().to_string();

        let created_at: sea_orm::prelude::DateTime = chrono::Utc::now().naive_utc();

        let user_model: ActiveModel = ActiveModel {
            name: Set(name),
            uuid: Set(u),
            username: Set(username),
            password_hash: Set(password_hash),
            email: Set(email),
            created_at: Set(created_at),
            ..Default::default()
        };

        let result: InsertResult<ActiveModel> = User::insert(user_model)
        .exec(self.connection)
        .await?;

        Ok(result)
    }


}