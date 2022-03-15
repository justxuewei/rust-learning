use generics_trait_lifecycle::generics;
use generics_trait_lifecycle::traits;
use generics_trait_lifecycle::lifetimes;

fn main() {
    generics::largest_test();
    generics::impl_generics_test();

    traits::trait_test();
    traits::trait_as_parameter_test();

    lifetimes::explicit_lifetimes_test();
}
