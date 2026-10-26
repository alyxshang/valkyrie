/*
VALKYRIE by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the
/// structure to
/// utilize a number
/// of simultaneous
/// connections to 
/// a database.
use sqlx::Pool;

/// Importing the macro
/// to execute database
/// queries that return
/// nothing.
use sqlx::query;

/// Importing the function to
/// hash a string with the 
/// BCrypt algorithm.
use bcrypt::hash;

/// Importing the function to
/// verify a hashed string with the 
/// BCrypt algorithm.
use bcrypt::verify;

/// Importing the macro
/// to execute database
/// queries that return
/// something.
use sqlx::query_as;

/// Importing the
/// function to generate
/// a SHA-256 hash from
/// a string.
use sha256::digest;

/// Importing the entity
/// to specify the cost
/// for hashing a string.
use bcrypt::DEFAULT_COST;

/// Importing the structure
/// that models a following-followee
/// relationship in the database.
use super::models::Follow;

/// Importing the function
/// to get the current time
/// in the RFC 2822 format.
use super::utils::rfc2822;

/// Importing the data structure
/// to catch and handle errors.
use super::err::ValkyrieErr;

/// Importing the data structure
/// that models information about
/// the current Valkyrie instance
/// in the database.
use super::models::Instance;

/// Importing the data structure
/// that models infoormation about
/// an API token for a user in the 
/// database.
use super::models::APIToken;

/// Importing the structure to
/// signal to the `Pool` structure
/// that PostgreSQL database is
/// being connected to.
use sqlx::postgres::Postgres;

/// Importing the entity from
/// the "rand" crate to generate
/// random alphanumeric strings.
use rand::distr::Alphanumeric;

/// Importing the entity from
/// the "rand" crate to generate
/// random alphanumeric sample 
/// strings.
use rand::distr::SampleString;

/// Importing the data structure
/// that models information about
/// a user in the database.
use super::models::ValkyrieUser;

/// Importing the data structure
/// that models information about
/// a post a user has made in the 
/// database.
use super::models::ValkyriePost;

/// A function to create a record
/// in the database for the current
/// Valkyrie instance. If the operation
/// succeeds, an instance of the `Instance`
/// model is returned. If the operation fails,
/// an error is returned.
pub async fn create_instance(
    host: &str,
    name: &str,
    admin: &str,
    use_https: &bool,
    description: &str,
    pool: &Pool<Postgres>
) -> Result<Instance, ValkyrieErr>{
    let instance: Instance = Instance{
        host: host.to_string(),
        name: name.to_string(),
        admin: admin.to_string(),
        use_https: *use_https,
        description: description.to_string(),
    };
    let _q = query!(
        "INSERT INTO instance (host, name, admin, use_https, description) VALUES ($1, $2, $3, $4, $5)",
        instance.host,
        instance.name,
        instance.admin,
        instance.use_https,
        instance.description
    ).execute(pool).await?;
    let fetched: Instance = get_instance(pool).await?;
    Ok(fetched)
}

/// A function to fetch the only record
/// in the database for the current
/// Valkyrie instance. If the operation
/// succeeds, an instance of the `Instance`
/// model is returned. If the operation fails,
/// an error is returned.
pub async fn get_instance(
    pool: &Pool<Postgres>
) -> Result<Instance, ValkyrieErr>{
    let results: Vec<Instance> = query_as!(
        Instance,
        "SELECT * FROM instance"
    ).fetch_all(pool).await?;
    if results.len() == 1{
        let result: Instance = match results.first(){
            Some(result) => result.clone(),
            None => return Err::<Instance, ValkyrieErr>(
                ValkyrieErr::new("Instance not found.")
            )
        };
        Ok(result)
    }
    else {
        Err::<Instance, ValkyrieErr>(
            ValkyrieErr::new("Error fetching instance: More than one.")
        )
    }
}

/// A function to update the only record
/// in the database for the current
/// Valkyrie instance. Specifically, 
/// the instance's name is updated. 
/// If the operation succeeds, nothing 
/// is returned. If the operation fails,
/// an error is returned.
pub async fn update_instance_name(
    new_name: &str,
    pool: &Pool<Postgres>
) -> Result<(), ValkyrieErr>{
    let instance: Instance = get_instance(pool).await?;
    let _q = query!(
        "UPDATE instance SET name = $1 WHERE name = $2",
        new_name,
        &instance.name
    ).execute(pool).await?;
    Ok(())
}

/// A function to update the only record
/// in the database for the current
/// Valkyrie instance. Specifically, 
/// the instance's description is updated. 
/// If the operation succeeds, nothing 
/// is returned. If the operation fails,
/// an error is returned.
pub async fn update_instance_description(
    new_description: &str,
    pool: &Pool<Postgres>
) -> Result<(), ValkyrieErr>{
    let instance: Instance = get_instance(pool).await?;
    let _q = query!(
        "UPDATE instance SET description = $1 WHERE name = $2",
        new_description,
        &instance.name
    ).execute(pool).await?;
    Ok(())
}

/// A function to create a record
/// in the database for a user of a
/// Valkyrie instance. If the operation
/// succeeds, an instance of the `ValkyrieUser`
/// model is returned. If the operation fails,
/// an error is returned.
pub async fn create_user(
    username: &str,
    password: &str,
    display_name: &str,
    description: &str,
    is_admin: bool,
    pool: &Pool<Postgres>
) -> Result<ValkyrieUser, ValkyrieErr>{
    let hashed_pwd: String = hash(password, DEFAULT_COST)?;
    let user: ValkyrieUser = ValkyrieUser {
        username: username.to_string(),
        password: hashed_pwd,
        display_name: display_name.to_string(),
        description: description.to_string(),
        is_admin: is_admin
    };
    let _q = query!(
        "INSERT INTO users (username, password, display_name, description, is_admin) VALUES ($1, $2, $3, $4, $5)",
        user.username,
        user.password,
        user.display_name,
        user.description,
        user.is_admin
    ).execute(pool).await?;
    let fetched: ValkyrieUser = get_user_by_name(
        username,
        pool
    ).await?;
    Ok(fetched)
}

/// A function to fetch a single record
/// in the database for a certain user.
/// If the operation succeeds, an instance 
/// of the `ValkyrieUser` model is returned. 
/// If the operation fails, an error is 
/// returned.
pub async fn get_user_by_name(
    username: &str,
    pool: &Pool<Postgres>
) -> Result<ValkyrieUser, ValkyrieErr>{
    let results: Vec<ValkyrieUser> = query_as!(
        ValkyrieUser,
        "SELECT * FROM users WHERE username = $1",
        username
    ).fetch_all(pool).await?;
    if results.len() == 1{
        let result: ValkyrieUser = match results.first(){
            Some(result) => result.clone(),
            None => return Err::<ValkyrieUser, ValkyrieErr>(
                ValkyrieErr::new(
                    &format!(
                        "Could not fetch user with name \"{}\"!", 
                        username
                    )
                )
            )
        };
        Ok(result)
    }
    else {
        Err::<ValkyrieUser, ValkyrieErr>(
            ValkyrieErr::new(
                &format!(
                    "Error fetching user \"{}\": More than one.", 
                    username
                )
            )
        )
    }
}

/// A function to update the record
/// in the database for a user. Specifically, 
/// the user's display name is updated. 
/// If the operation succeeds, nothing 
/// is returned. If the operation fails,
/// an error is returned.
pub async fn update_user_display_name(
    username: &str,
    new_display_name: &str,
    pool: &Pool<Postgres>
) -> Result<(), ValkyrieErr>{
    let user: ValkyrieUser = get_user_by_name(
        username,
        pool
    ).await?;
    let _q = query!(
        "UPDATE users SET display_name = $1 WHERE username = $2",
        new_display_name,
        &user.username
    ).execute(pool).await?;
    Ok(())
}

/// A function to update the record
/// in the database for a user. Specifically, 
/// the user's description is updated. 
/// If the operation succeeds, nothing 
/// is returned. If the operation fails,
/// an error is returned.
pub async fn update_user_description(
    username: &str,
    new_description: &str,
    pool: &Pool<Postgres>
) -> Result<(), ValkyrieErr>{
    let user: ValkyrieUser = get_user_by_name(
        username,
        pool
    ).await?;
    let _q = query!(
        "UPDATE users SET description = $1 WHERE username = $2",
        new_description,
        &user.username
    ).execute(pool).await?;
    Ok(())
}

/// A function to update the record
/// in the database for a user. Specifically, 
/// the user's password is updated. 
/// If the operation succeeds, nothing 
/// is returned. If the operation fails,
/// an error is returned.
pub async fn update_user_password(
    username: &str,
    old_password: &str,
    new_password: &str,
    pool: &Pool<Postgres>
) -> Result<(), ValkyrieErr>{
    let user: ValkyrieUser = get_user_by_name(
        username,
        pool
    ).await?;
    let verified: bool = verify(old_password, &user.password)?;
    if verified {
        let hashed_pwd: String = hash(new_password, DEFAULT_COST)?;
        let _q = query!(
            "UPDATE users SET password = $1 WHERE username = $2",
            hashed_pwd,
            &user.username
        ).execute(pool).await?;
        return Ok(());
    }
    Err::<(), ValkyrieErr>(
        ValkyrieErr::new(
            &format!(
                "Could not verify password for user \"{}\"!", 
                username
            )
        )
    )
}

/// A function to delete the record
/// in the database for a user. 
/// If the operation succeeds, nothing 
/// is returned. If the operation fails,
/// an error is returned.
pub async fn delete_user(
    username: &str,
    pool: &Pool<Postgres>
) -> Result<(), ValkyrieErr>{
    let user: ValkyrieUser = get_user_by_name(
        username,
        pool
    ).await?;
    let _q = query!(
        "DELETE FROM users WHERE username = $1",
        &user.username
    ).execute(pool).await?;
    Ok(())
}

/// A function to create a record
/// in the database for a user's API token
/// If the operation succeeds, an instance 
/// of the `APIToken` model is returned. 
/// If the operation fails, an error is 
/// returned.
pub async fn create_api_token(
    username: &str,
    password: &str,
    pool: &Pool<Postgres>
) -> Result<APIToken, ValkyrieErr>{
    let user: ValkyrieUser = get_user_by_name(
        username,
        pool
    ).await?;
    let verified: bool = verify(password, &user.password)?;
    if verified {
        let token_str_source: String = format!(
            "{}{}{}{}",
            rfc2822(),
            user.username,
            user.password,
            Alphanumeric.sample_string(&mut rand::rng(), 32)
        );
        let token_str: String = digest(token_str_source);
        let token: APIToken = APIToken {
            owner: user.username.clone(),
            token: token_str.clone()
        };
        let _q = query!(
            "INSERT INTO api_tokens (owner, token) VALUES ($1, $2)",
            token.owner,
            token.token
        ).execute(pool).await?;
        let fetched: APIToken = get_token_by_token(
            &token_str,
            pool
        ).await?;
        return Ok(fetched);

    }
    Err::<APIToken, ValkyrieErr>(
        ValkyrieErr::new(
            &format!(
                "Could not verify password for user \"{}\"!", 
                username
            )
        )
    )
}

/// A function to fetch a single record
/// in the database for a certain API token.
/// If the operation succeeds, an instance 
/// of the `APIToken` model is returned. 
/// If the operation fails, an error is 
/// returned.
pub async fn get_token_by_token(
    token: &str,
    pool: &Pool<Postgres>
) -> Result<APIToken, ValkyrieErr>{
    let results: Vec<APIToken> = query_as!(
        APIToken,
        "SELECT * FROM api_tokens WHERE token = $1",
        token
    ).fetch_all(pool).await?;
    if results.len() == 1{
        let result: APIToken = match results.first(){
            Some(result) => result.clone(),
            None => return Err::<APIToken, ValkyrieErr>(
                ValkyrieErr::new(
                    "Could not fetch API token."
                )
            )
        };
        Ok(result)
    }
    else {
        Err::<APIToken, ValkyrieErr>(
            ValkyrieErr::new(
                "Error fetching API token: More than one." 
            )
        )
    }
}

/// A function to delete the record
/// in the database for an API token. 
/// If the operation succeeds, nothing 
/// is returned. If the operation fails,
/// an error is returned.
pub async fn delete_api_token(
    token: &str,
    pool: &Pool<Postgres>
) -> Result<(), ValkyrieErr>{
    let object: APIToken = get_token_by_token(
        token,
        pool
    ).await?;
    let _q = query!(
        "DELETE FROM api_tokens WHERE token = $1",
        &object.token
    ).execute(pool).await?;
    Ok(())
}

/// A function to create a record
/// in the database for a user's new post
/// If the operation succeeds, an instance 
/// of the `ValkyriePost` model is returned. 
/// If the operation fails, an error is 
/// returned.
pub async fn create_post(
    post_id: &Option<&str>,
    created_at: &Option<&str>,
    username: &str,
    host: &Option<&str>,
    content: &str,
    pool: &Pool<Postgres>
) -> Result<ValkyriePost, ValkyrieErr>{
    let user: ValkyrieUser = get_user_by_name(
        username,
        pool
    ).await?; 
    let id: String = match post_id {
        Some(f) => f.to_string(),
        None => {
            let post_id_src: String = format!(
                "{}{}{}{}",
                content,
                user.username,
                rfc2822(),
                user.password
            );
            digest(post_id_src)
        }
    };
    let date: String = match created_at {
        Some(ca) => ca.to_string(),
        None => rfc2822()
    };
    let created_by_host: String = match host{
        Some(f) => f.to_string(),
        None => get_instance(pool).await?.host
    };
    let post: ValkyriePost = ValkyriePost {
        post_id: id.clone(),
        created_at: date,
        created_by: user.username.clone(),
        created_by_host: created_by_host.clone(),
        post_content: content.to_string()
    };
    let _q = query!(
        "INSERT INTO posts (post_id, created_at, created_by, created_by_host, post_content) VALUES ($1, $2, $3, $4, $5)",
        post.post_id,
        post.created_at,
        post.created_by,
        post.created_by_host,
        post.post_content
    ).execute(pool).await?;
    let fetched: ValkyriePost = get_post_by_id(
        &id,
        &created_by_host,
        pool
    ).await?;
    Ok(fetched)
}

/// A function to fetch a single record
/// in the database for a certain post.
/// If the operation succeeds, an instance 
/// of the `ValkyriePost` model is returned. 
/// If the operation fails, an error is 
/// returned.
pub async fn get_post_by_id(
    post_id: &str,
    host: &str,
    pool: &Pool<Postgres>
) -> Result<ValkyriePost, ValkyrieErr>{
    let results: Vec<ValkyriePost> = query_as!(
        ValkyriePost,
        "SELECT * FROM posts WHERE post_id = $1 AND created_by_host = $2",
        post_id,
        host
    ).fetch_all(pool).await?;
    if results.len() == 1{
        let result: ValkyriePost = match results.first(){
            Some(result) => result.clone(),
            None => return Err::<ValkyriePost, ValkyrieErr>(
                ValkyrieErr::new(
                    &format!(
                        "Could not fetch post with the ID \"{}\"!",
                        post_id
                    )
                )
            )
        };
        Ok(result)
    }
    else {
        Err::<ValkyriePost, ValkyrieErr>(
            ValkyrieErr::new(
                &format!(
                    "Error fetching post \"{}\": More than one.",
                    post_id
                )
            )
        )
    }
}

/// A function to delete the record
/// in the database for a user-created post. 
/// If the operation succeeds, nothing 
/// is returned. If the operation fails,
/// an error is returned.
pub async fn delete_post(
    post_id: &str,
    host: &str,
    pool: &Pool<Postgres>
) -> Result<(), ValkyrieErr>{
    let object: ValkyriePost = get_post_by_id(
        post_id,
        host,
        pool
    ).await?;
    let _q = query!(
        "DELETE FROM posts WHERE post_id = $1 AND created_by_host = $2",
        &object.post_id,
        host
    ).execute(pool).await?;
    Ok(())
}

/// A function to update the record
/// in the database for a user-created 
/// post. Specifically, the post's 
/// content is updated. If the operation 
/// succeeds, nothing is returned. If 
/// the operation fails, an error is 
/// returned.
pub async fn update_post(
    post_id: &str,
    host: &str,
    new_content: &str,
    pool: &Pool<Postgres>
) -> Result<(), ValkyrieErr>{
    let post: ValkyriePost = get_post_by_id(
        post_id,
        host,
        pool
    ).await?;
    let _q = query!(
        "UPDATE posts SET post_content = $1 WHERE post_id = $2",
        new_content,
        &post.post_id
    ).execute(pool).await?;
    Ok(())
}

/// A function to retrieve the
/// record for a Valkyrie user
/// given one of the user's API
/// tokens. If the operation is
/// successful, an instance of
/// the `ValkyrieUser` structure
/// is returned. If the operation
/// fails, an error is returned.
pub async fn get_user_by_token(
    token: &str,
    pool: &Pool<Postgres>
) -> Result<ValkyrieUser, ValkyrieErr>{
    let token: APIToken = get_token_by_token(
        token,
        pool
    ).await?;
    let user: ValkyrieUser = get_user_by_name(
        &token.owner,
        pool
    ).await?;
    Ok(user)
}

/// A function to retrieve all the
/// records from the database for 
/// posts a user has created. If the
/// operation is successful, an instance
/// of the `Vec` structure is returned
/// containing instances of the `ValkyriePost`
/// structure. If the operation fails, an error
/// is returned.
pub async fn get_user_posts(
    username: &str,
    host: &str,
    pool: &Pool<Postgres>
) -> Result<Vec<ValkyriePost>, ValkyrieErr>{
    let posts: Vec<ValkyriePost> = query_as!(
        ValkyriePost,
        "SELECT * FROM posts WHERE created_by = $1 AND created_by_host = $2",
        username,
        host
    ).fetch_all(pool).await?;
    Ok(posts)
}

/// A function to retrieve all the
/// records from the database for 
/// API tokens a user has created. If the
/// operation is successful, an instance
/// of the `Vec` structure is returned
/// containing instances of the `APIToken`
/// structure. If the operation fails, an error
/// is returned.
pub async fn get_user_tokens(
    username: &str,
    pool: &Pool<Postgres>
) -> Result<Vec<APIToken>, ValkyrieErr>{
    let user: ValkyrieUser = get_user_by_name(
        username,
        pool
    ).await?;
    let tokens: Vec<APIToken> = query_as!(
        APIToken,
        "SELECT * FROM api_tokens WHERE owner = $1",
        &user.username
    ).fetch_all(pool).await?;
    Ok(tokens)
}

/// A function to check whether
/// a record for a user exists
/// in the database. A boolean
/// is returned to reflect this.
pub async fn user_exists(
    username: &str,
    pool: &Pool<Postgres>
) -> bool {
    get_user_by_name(username, pool).await.is_ok()
}

/// A function to create a record
/// in the database for a following-followee
/// relationship between two users.
/// If the operation succeeds, an instance 
/// of the `Follow` model is returned. 
/// If the operation fails, an error is 
/// returned.
pub async fn create_follow(
    id: &Option<&str>,
    sender_host: &str,
    sender_handle: &str,
    receiver_host: &str,
    receiver_handle: &str,
    pool: &Pool<Postgres>
) -> Result<Follow, ValkyrieErr>{
    if (receiver_handle == sender_handle &&
       sender_host != receiver_host) ||
       (receiver_handle != sender_handle &&
        sender_host == receiver_host)
    {
        let instance_info: Instance = get_instance(pool)
            .await?;
        let f_id: String = if receiver_host == instance_info.host && 
            sender_host != instance_info.host{
            let rel_id: String = match id{
                Some(rel_id) => rel_id.to_string(),
                None => return Err::<Follow, ValkyrieErr>(
                    ValkyrieErr::new("No relationship ID specified.")
                )
            };
            rel_id
        }
        else {
            let id_source: String = format!(
                "{}{}{}{}{}", 
                rfc2822(),
                sender_host,
                sender_handle,
                receiver_host,
                receiver_handle
            );
            digest(&id_source)
        };
        let rel: Follow = Follow{
            id: f_id,
            sender_host: sender_host.to_string(),
            sender_handle: sender_handle.to_string(),
            receiver_host: receiver_host.to_string(),
            receiver_handle: receiver_handle.to_string()
        };
        let _io = query!(
            "INSERT INTO follows (id, sender_host, sender_handle, receiver_host, receiver_handle) VALUES ($1, $2, $3, $4, $5)",
            rel.id,
            rel.sender_host,
            rel.sender_handle,
            rel.receiver_host,
            rel.receiver_handle
        ).execute(pool).await?;
        let fetched: Follow = get_follow_by_participants(
            &rel.sender_host,
            &rel.sender_handle,
            &rel.receiver_host,
            &rel.receiver_handle,
            pool
        ).await?;
        Ok(fetched)
    }
    else {
        Err::<Follow, ValkyrieErr>(
            ValkyrieErr::new("Sender and receiver cannot be the same user on the same instance.")
        )
    }
}

/// A function to fetch a single record
/// in the database for a certain 
/// following-followee relationship.
/// If the operation succeeds, an instance 
/// of the `Follow` model is returned. 
/// If the operation fails, an error is 
/// returned.
pub async fn get_follow_by_participants(
    sender_host: &str,
    sender_handle: &str,
    receiver_host: &str,
    receiver_handle: &str,
    pool: &Pool<Postgres>
) -> Result<Follow, ValkyrieErr>{
    let results: Vec<Follow> = query_as!(
        Follow,
        "SELECT * FROM follows WHERE sender_host = $1 AND sender_handle = $2 AND receiver_host = $3 AND receiver_handle = $4",
        sender_host,
        sender_handle,
        receiver_host,
        receiver_handle,
    ).fetch_all(pool).await?;
    if results.len() == 1{
        let result: Follow = match results.first(){
            Some(result) => result.clone(),
            None => return Err::<Follow, ValkyrieErr>(
                ValkyrieErr::new(
                    "Could not fetch relationship between users!",
                )
            )
        };
        Ok(result)
    }
    else {
        Err::<Follow, ValkyrieErr>(
            ValkyrieErr::new(
                "Error fetching user relationship: More than one.",
            )
        )
    }
}

/// A function to delete a single record
/// in the database for a certain 
/// following-followee relationship.
/// If the operation succeeds, nothing 
/// is returned. If the operation fails, 
/// an error is returned.
pub async fn delete_follow_by_participants(
    sender_host: &str,
    sender_handle: &str,
    receiver_host: &str,
    receiver_handle: &str,
    pool: &Pool<Postgres>
) -> Result<(), ValkyrieErr>{
    let fetched: Follow = get_follow_by_participants(
        sender_host,
        sender_handle,
        receiver_host,
        receiver_handle,
        pool
    ).await?;
    let _q = query!(
        "DELETE FROM follows WHERE id = $1",
        &fetched.id
    ).execute(pool).await?;
    Ok(())
}

/// A function to retrieve records for
/// all the users a user is following. If
/// the operation is successful, a vector of
/// instances of the `Follow` structure
/// is returned. If the operation fails, an 
/// error is returned.
pub async fn get_user_following(
    username: &str,
    pool: &Pool<Postgres>
) -> Result<Vec<Follow>, ValkyrieErr>{
    let user: ValkyrieUser = get_user_by_name(
        username,
        pool
    ).await?;
    let instance: Instance = get_instance(pool)
        .await?;
    let queried: Vec<Follow> = query_as!(
        Follow,
        "SELECT * FROM follows WHERE sender_handle = $1 AND sender_host =$2",
        &user.username,
        &instance.host
    ).fetch_all(pool).await?;
    Ok(queried)
}

/// A function to retrieve records for
/// all the users a user is followed by. If
/// the operation is successful, a vector of
/// instances of the `Follow` structure
/// is returned. If the operation fails, an 
/// error is returned.
pub async fn get_user_followers(
    username: &str,
    pool: &Pool<Postgres>
) -> Result<Vec<Follow>, ValkyrieErr>{
    let user: ValkyrieUser = get_user_by_name(
        username,
        pool
    ).await?;
    let instance: Instance = get_instance(pool)
        .await?;
    let queried: Vec<Follow> = query_as!(
        Follow,
        "SELECT * FROM follows WHERE receiver_handle = $1 AND receiver_host =$2",
        &user.username,
        &instance.host
    ).fetch_all(pool).await?;
    Ok(queried)
}
