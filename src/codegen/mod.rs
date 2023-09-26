mod function;
mod interface;
mod record;
mod variant;

pub use function::*;
pub use interface::*;
pub use record::*;
pub use variant::*;

use color_eyre::Result;

/// The `Render` trait represents things that can be rendered to a `String`
pub trait Render {
    /// Renders this to a `String`
    fn render(self) -> Result<String>;
}
