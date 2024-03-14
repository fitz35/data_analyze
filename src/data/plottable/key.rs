use std::fmt::Display;

/// Define a trait for a key of a serie
pub trait SerieKey : Eq + std::hash::Hash + Copy + Display + Ord + Send + Sync{
    /// get the display name of the serie
    fn get_display_name(&self) -> String;

    /// if the serie is numeric
    fn is_numeric(&self) -> bool;

    /// if the serie is a string
    fn is_string(&self) -> bool;

    /// if the serie is an object
    fn is_object(&self) -> bool;
}


/// Define a fully initialized key for a serie
/// Warn : must be called only once by file
/// Need the dependances :
/// ```
/// use serde_derive::{Deserialize, Serialize};
/// use std::fmt::{Display, Formatter};
/// use plot_helper::generate_plot_key;
/// use plot_helper::data::plottable::key::SerieKey;
/// generate_plot_key!(
///     MultiLineQueryKey[
///         ParsingTime { "parsing time (s)", Numeric},
///         ParsingMaxMemory { "parsing max memory (Mb)", Numeric }
///     ],
///     SingleLineQueryKey[
///         ParsingTime { "parsing time (s)", Numeric },
///         File { "file", String }
///     ]
/// );
/// assert_eq!(MultiLineQueryKey::ParsingTime.get_display_name(), "parsing time (s)");
/// assert_eq!(MultiLineQueryKey::ParsingTime.is_numeric(), true);
/// ```
#[macro_export]
macro_rules! generate_plot_key {
    ($($key_name:ident [ 
        $($variant:ident {
             $description:literal, $key_type:ident
        }),* 
    ]),+) => {
        /// Define the type of the key
        #[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
        enum KeyType {
            /// The key is numeric
            Numeric,
            /// The key is a string
            String,
            /// The key is an object
            Object
        }

        /// Define the keys

        $(
            #[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Copy)]
            pub enum $key_name {
                $($variant),*
            }

            impl SerieKey for $key_name {
                fn get_display_name(&self) -> String {
                    match self {
                        $($key_name::$variant => $description.to_string()),*
                    }
                }
                fn is_numeric(&self) -> bool {
                    match self {
                        $($key_name::$variant => KeyType::$key_type == KeyType::Numeric),*
                    }
                }
                fn is_string(&self) -> bool {
                    match self {
                        $($key_name::$variant => KeyType::$key_type == KeyType::String),*
                    }
                }
                fn is_object(&self) -> bool {
                    match self {
                        $($key_name::$variant => KeyType::$key_type == KeyType::Object),*
                    }
                }
            }

            impl Display for $key_name {
                fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", self.get_display_name())
                }
            }
            
            impl PartialOrd for $key_name {
                fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                    self.get_display_name().partial_cmp(&other.get_display_name())
                }
            }
            
            impl Ord for $key_name {
                fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                    self.get_display_name().cmp(&other.get_display_name())
                }
            }
        )*
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_derive::{Deserialize, Serialize};
    use std::fmt::{Display, Formatter};
    #[test]
    fn test_generate_plot_key() {
        generate_plot_key!(TestKey[
            A { "A", Numeric },
            B { "B", String },
            C { "C", Object }
        ]);

        let key = TestKey::A;
        assert_eq!(key.get_display_name(), "A");
        assert_eq!(key.is_numeric(), true);
        assert_eq!(key.is_string(), false);
        assert_eq!(key.is_object(), false);

        let key = TestKey::B;
        assert_eq!(key.get_display_name(), "B");
        assert_eq!(key.is_numeric(), false);
        assert_eq!(key.is_string(), true);
        assert_eq!(key.is_object(), false);

        let key = TestKey::C;

        assert_eq!(key.get_display_name(), "C");
        assert_eq!(key.is_numeric(), false);
        assert_eq!(key.is_string(), false);
        assert_eq!(key.is_object(), true);
    }



}