use rand::seq::SliceRandom;
use rand::rngs::ThreadRng;
use rand::thread_rng;

pub fn create_fruit_salad(num_fruits: usize) -> Vec<String> {
    let fruits: Vec<String> = vec![
        "Arbutus".to_string(),
        "Peach".to_string(),
        "Pear".to_string(),
        "Apple".to_string(),
        "Orange".to_string(),
        "Strawberry".to_string(),
        "Fig".to_string(),
        "Loquat".to_string(),
    ];

    let mut rng: ThreadRng = thread_rng();
    fruits.shuffle(&mut rng);

    return fruits;
}
