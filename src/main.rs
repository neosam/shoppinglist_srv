#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate shoppinglist;
extern crate serde_json;
extern crate uuid;
#[macro_use] extern crate serde_derive;

use rocket::State;
use rocket::response::content::Content;
use rocket::http::ContentType;
use shoppinglist::*;
use std::fs::File;
use std::io::Read;

use uuid::Uuid;

use std::cell::RefCell;
use std::sync::Mutex;

type DefaultState = Mutex<RefCell<ShoppingList>>;

#[get("/")]
fn index() -> &'static str {
    "Hello world"
}

#[derive(Serialize)]
struct RecipeIngredient {
    ingredient: Ingredient,
    amount: f32
}
#[derive(Serialize)]
struct RecipeResponse {
    key: Uuid,
    name: String,
    ingredients: Vec<RecipeIngredient>
}

#[get("/get-recipes")]
fn get_recipes(shoppinglist: State<DefaultState>) -> String {
    let shoppinglist = shoppinglist.lock().unwrap();
    let shoppinglist = shoppinglist.borrow();
    let recipes : Vec<RecipeResponse> = shoppinglist.recipe_iter()
        .map(|x : &Recipe | RecipeResponse {
            key: x.key.clone(),
            name: x.name.clone(),
            ingredients: x.ingredients.iter().map(|&(ingredient_id, amount)| RecipeIngredient {
                ingredient: shoppinglist.ingredient(Key::new(ingredient_id)).clone(),
                amount
            }).collect()
        })
        .collect();
    serde_json::to_string(&recipes).unwrap()
}

#[derive(FromForm)]
struct AddRecipeForm {
    name: String
}

#[get("/add-recipe?<form>")]
fn add_recipe(shoppinglist: State<DefaultState>, form: AddRecipeForm) -> String {
    let shoppinglist = shoppinglist.lock().unwrap();
    let mut shoppinglist = shoppinglist.borrow_mut();
    let key = shoppinglist.generate_recipe(form.name);
    serde_json::to_string(&key.key).unwrap()
}

#[derive(Serialize)]
struct ShoppingListResponse {
    pub key: Uuid,
    pub name: String,
    pub ingredient_key: Uuid,
    pub amount: f32
}

#[get("/get-shoppinglist")]
fn get_shoppinglist(shoppinglist: State<DefaultState>) -> String {
    let shoppinglist = shoppinglist.lock().unwrap();
    let shoppinglist = shoppinglist.borrow();
    let recipes : Vec<ShoppingListResponse> = shoppinglist.shoppinglist_iter()
        .map(|x : &ShoppingListItem| ShoppingListResponse {
            key: x.key.clone(),
            name: shoppinglist.ingredient(Key::new(x.ingredient_key)).name.clone(),
            ingredient_key: x.ingredient_key.clone(),
            amount: x.amount
        })
        .collect();
    serde_json::to_string(&recipes).unwrap()
}

#[derive(FromForm)]
struct AddShoppinglistForm {
    ingredient_key: String,
    amount: f32
}

#[get("/add-shoppinglist?<form>")]
fn add_shoppinglist(shoppinglist: State<DefaultState>, form: AddShoppinglistForm) -> String {
    let shoppinglist = shoppinglist.lock().unwrap();
    let mut shoppinglist = shoppinglist.borrow_mut();
    let ingredient_key = Uuid::parse_str(&form.ingredient_key).unwrap();
    let key =
        shoppinglist.insert_shoppinglist(Key::new(ingredient_key), form.amount);
    serde_json::to_string(&key.key).unwrap()
}

#[derive(FromForm)]
struct AddIngredientForm {
    name: String
}


#[get("/add-ingredient?<form>")]
fn add_ingredient(shoppinglist: State<DefaultState>, form: AddIngredientForm) -> String {
    let shoppinglist = shoppinglist.lock().unwrap();
    let mut shoppinglist = shoppinglist.borrow_mut();
    let key = shoppinglist.insert_ingredient(form.name);
    serde_json::to_string(&key.key).unwrap()
}

#[derive(FromForm)]
struct AddIngredientToRecipeForm {
    recipe_key: String,
    ingredient_key: String,
    amount: f32
}

#[get("/get-ingredients")]
fn get_ingredients(shoppinglist: State<DefaultState>) -> String {
    let shoppinglist = shoppinglist.lock().unwrap();
    let shoppinglist = shoppinglist.borrow();
    let result : Vec<Ingredient> = shoppinglist
        .ingredient_iter()
        .map(|x| x.clone())
        .collect();
    serde_json::to_string(&result).unwrap()
}

#[get("/add-ingredient-to-recipe?<form>")]
fn add_ingredient_to_recipe(shoppinglist: State<DefaultState>, form: AddIngredientToRecipeForm) -> &str {
    let shoppinglist = shoppinglist.lock().unwrap();
    let mut shoppinglist = shoppinglist.borrow_mut();
    let recipe = shoppinglist.recipe_mut(Key::new(Uuid::parse_str(&form.recipe_key).unwrap()));
    recipe.add_ingredient(Key::new(Uuid::parse_str(&form.ingredient_key).unwrap()), form.amount);
    "true"
}

#[get("/file/<path>")]
fn file(path: String) -> Content<Vec<u8>> {
    let mut result = Vec::new();
    let path = format!("files/{}", path);
    File::open(path.clone()).unwrap().read_to_end(&mut result);
    let content_type = if path.ends_with(".html") {
            ContentType::new("text", "html")
        } else if (path.ends_with(".js")) {
            ContentType::new("text", "javascript")
        } else if (path.ends_with(".css")) {
            ContentType::new("text", "css")
        } else if (path.ends_with(".jpg")) {
            ContentType::new("image", "jpeg")
        } else if (path.ends_with(".png")) {
            ContentType::new("image", "png")
        } else {
            ContentType::new("text", "plain")
        };
    Content(content_type, result)
}


fn main() {
    let r = rocket::ignite();
    let shoppinglist = ShoppingList::new();
    let r = r.manage(Mutex::new(RefCell::new(shoppinglist)));
    let r = r.mount("/", routes![
        index,
        get_recipes,
        add_recipe,
        add_ingredient,
        get_ingredients,
        add_ingredient_to_recipe,
        get_shoppinglist,
        add_shoppinglist,
        file]);
    r.launch();
}
