use parry2d::query;
use parry2d::math::Isometry;
use parry2d::shape::Shape;

pub trait Bounds {
    fn as_parry(&self) -> (Isometry<f32>, Box<dyn Shape>);

    fn colliding(&self, other: &dyn Bounds) -> bool {
        let (iso, shape) = self.as_parry();
        let (other_iso, other_shape) = other.as_parry();

        query::contact(&iso, &*shape, &other_iso, &*other_shape, 0.0)
            .is_ok_and(|target| target.is_some())
    }
}
