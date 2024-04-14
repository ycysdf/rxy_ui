use vello::kurbo::{Point, Rect, Shape};

use crate::{Either, EitherExt};

impl<T1, T2> Shape for Either<T1, T2>
where
   T1: Shape,
   T2: Shape,
{
   type PathElementsIter<'iter>
   where
      Self: 'iter,
   = Either<T1::PathElementsIter<'iter>, T2::PathElementsIter<'iter>>;

   fn path_elements(&self, tolerance: f64) -> Self::PathElementsIter<'_> {
      match self {
         Either::Left(n) => n.path_elements(tolerance).either_left(),
         Either::Right(n) => n.path_elements(tolerance).either_right(),
      }
   }

   fn area(&self) -> f64 {
      match self {
         Either::Left(n) => n.area(),
         Either::Right(n) => n.area(),
      }
   }

   fn perimeter(&self, accuracy: f64) -> f64 {
      match self {
         Either::Left(n) => n.perimeter(accuracy),
         Either::Right(n) => n.perimeter(accuracy),
      }
   }

   fn winding(&self, pt: Point) -> i32 {
      match self {
         Either::Left(n) => n.winding(pt),
         Either::Right(n) => n.winding(pt),
      }
   }

   fn bounding_box(&self) -> Rect {
      match self {
         Either::Left(n) => n.bounding_box(),
         Either::Right(n) => n.bounding_box(),
      }
   }
}
