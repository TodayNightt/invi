mod error_descriptor;

pub use error_descriptor::ErrorDescriptor;

pub fn get_type_name<T>(_ :T) -> &'static str {
    std::any::type_name::<T>()
}
