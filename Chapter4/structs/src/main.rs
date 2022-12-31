struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn main(){
    let mut user1 = User{
        email: String::from("jaswanthraj@rust.com"),
        username: String::from("Jaswanthraj"),
        active: true,
        sign_in_count: 1
    };

    let _name = user1.username;
    user1.username = String::from("Jas:)");

    let user2: User = build_user(String::from("Rust Test User1"), String::from("testUser1@rust.com"));

    // Struct Update Syntax
    let user3: User = User{
        username: String::from("Rust Test User2"),
        email: String::from("testUser2@rust.com"),
        ..user2
    };

}

// Create User struct
fn build_user(username: String, email: String) -> User{
    User{
        //Field Init Shorthand
        username, 
        email,
        active: true,
        sign_in_count: 1
    }
}
