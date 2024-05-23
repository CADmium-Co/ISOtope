pub mod line;
pub mod arc;
pub mod circle;

use self::line::Line;
use self::arc::Arc;
use self::circle::Circle;

pub enum SketchPrimitives{
    Line(Line),
    Arc(Arc),
    Circle(Circle),
}