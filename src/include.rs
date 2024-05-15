use autocxx::include_cpp;
include_cpp! {
    #include "coresystem/task.hpp"
    safety!(unsafe) // see details of unsafety policies described in the 'safety' section of the book
    generate!("CSEzTask") // add this line for each function or type you wish to generate
}

