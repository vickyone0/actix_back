// use diesel::prelude::*;

// // use crate::schema::posts::title;


// use crate::schema::products::{self, price};
// use chrono::NaiveDateTime;
// use chrono::Utc;


// #[derive(Queryable, Selectable)]
// #[diesel(table_name = crate::schema::products)]
// pub struct Product{
//     pub id: i32,
//     pub name: String,
//     pub price: i32, 
//     pub created_at: chrono::NaiveDateTime

// }

// #[derive(Insertable)]
// #[diesel(table_name = crate::schema::products)]
// pub struct NewProduct {
//     pub name: String,
//     pub price: i32,
// }
// // #[derive(Insertable)]
// // #[diesel(table_name = crate::schema::products)]
// // pub struct NewProduct{
// //     pub name: String,
// //     pub price: i32, 
// //     pub created_at: chrono::NaiveDateTime

// // }




// // #[derive(Queryable, Selectable)]
// // #[diesel(table_name = crate::schema::users)]
// // pub struct User {
// //     pub id: i32,
// //     pub first_name: String,
// //     pub last_name: String,
// //     pub username: String,
// //     pub email: String,
// //     pub password: String,
// //     pub created_at: chrono::NaiveDateTime,
// // }

// // #[derive(Insertable)]
// // #[diesel(table_name = crate::schema::users)]
// // pub struct NewUser {
// //     pub username: String,
// //     pub first_name: String,
// //     pub last_name: String,
// //     pub email: String,
// //     pub password: String,
// // }

// // #[derive(Queryable, Selectable)]
// // #[diesel(table_name = crate::schema::posts)]
// // pub struct Post {
// //     pub id: i32,
// //     pub title: String,
// //     pub body: String,
// //     pub user_id: i32,
// //     pub created_at: chrono::NaiveDateTime,
// //     pub catagory_id: Option<i32>,
// //     pub published: bool,
// // }

// // #[derive(Insertable)]
// // #[diesel(table_name = crate::schema::posts)]
// // pub struct NewPost {
// //     pub title: String,
// //     pub body: String,
// //     pub user_id: i32,
// //     pub catagory_id: Option<i32>,
// //     pub published: bool,
// // }

// // #[derive(Queryable, Selectable)]
// // #[diesel(table_name = crate::schema::categories)]
// // pub struct Category {
// //     pub id: i32,
// //     pub name: String,
// // }


// pub fn create_product(conn: &mut PgConnection, product_name: &str , product_price:i32) -> NewProduct {
//     use crate::schema::products::dsl::*;

//     let new_product = NewProduct{
//         name: product_name.to_string(),
//         price: product_price,
//     };
//     diesel::insert_into(products)
//         .values(&new_product)
//         .execute(conn)
//         .expect("Error in saving the value");

//     new_product

// }

// // pub fn update_post(conn: &mut PgConnection, post_id: i32, new_title: &str) -> Result<Post, diesel::result::Error> {
// //     use crate::schema::posts::dsl::*;

// //     let updated_post = diesel::update(posts.filter(id.eq(post_id)))
// //         .set((title.eq(new_title)))
// //         .get_result::<Post>(conn)?;

// //     Ok(updated_post)
// // }

// // pub fn delete_post(conn: &mut PgConnection, post_title: String) -> Result<usize, diesel::result::Error> {
// //     use crate::schema::posts::dsl::*;

// //     let deleted_count = diesel::delete(posts.filter(title.eq(post_title))).execute(conn)?;

// //     Ok(deleted_count)
// // }

// // pub fn read_posts(conn: &mut PgConnection, limit: i64, offset: i64) -> Result<Vec<Post>, diesel::result::Error> {
// //     use crate::schema::posts::dsl::*;

// //     let results = posts
// //         .limit(limit)
// //         .offset(offset)
// //         .load::<Post>(conn)?;

// //     Ok(results)
// // }