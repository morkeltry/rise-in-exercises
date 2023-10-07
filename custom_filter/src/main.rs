
//  Define a struct called FilterCondition with a single field of the desired type for filtering.
//  Implement a method called is_match on the FilterCondition struct that takes
//   a reference to an item of the same type as the filter condition and returns a boolean indicating whether the item matches the condition.
//  Define a function called custom_filter that takes a collection (e.g., a vector) and a reference to a FilterCondition object as arguments.
//  The function should iterate over the elements in the collection and return a new collection containing only 
//   the elements that match the filter condition.
//  In the main function, create a collection (e.g., a vector) with some elements and initialize a FilterCondition object 
//  with the desired value.
//  Call the custom_filter function with the collection and the FilterCondition object, storing the result in a new variable.
//  Print the filtered result to the console.
//  Compile and run the program to test its functionality.


// Create a new Rust project and open the main.
//  rs file.
//  Define the FilterCondition struct with the desired type for filtering.
//  Implement the is_match method on the FilterCondition struct.
//  Define the custom_filter function to filter elements based on the condition.
//  Create a collection and a FilterCondition object in the main function.
//  Call the custom_filter function and store the result.
//  Print the filtered result to the console.
//  Compile and run the program to test its functionality.



#[derive(Debug)]
pub struct FilterCondition {
    hamsters : i32,
}

impl FilterCondition {
    fn is_match (&self, el: &FilterCondition) -> bool {
        el.hamsters > 4
    }
}
//                                               I don't understand why this function should take a second argument,
//                                               just so that it can use the is_match which is common to all struct instances.
pub fn custom_filter (vec: Vec<FilterCondition>, filter_condition: &FilterCondition) -> Vec<FilterCondition>{
    vec.into_iter()
        .filter(|el| filter_condition.is_match(el))
        .collect()
}

fn main() {
    let test_vec: Vec<FilterCondition> = Vec::from([
        FilterCondition { hamsters: 2 },
        FilterCondition { hamsters: 4 },
        FilterCondition { hamsters: 8 },
        FilterCondition { hamsters: 2048},
    ]);

    let my_filter_condition = FilterCondition {hamsters: 23} ;

    let filtered_result = custom_filter(test_vec, &my_filter_condition);

    println!("{:?}", filtered_result);
}
