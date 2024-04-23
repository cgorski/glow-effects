use crate::util::color::ColorContainer;
use crate::util::color_point::ColorPointContainer;

pub trait Effect<U: ColorContainer, T: ColorPointContainer<U>> {
    fn get_frame(&mut self) -> Vec<T>;

    fn iter(&mut self) -> EffectIterator<U, T, Self> {
        EffectIterator {
            effect: self,
            phantom_color_value: std::marker::PhantomData,
            phantom_color_point: std::marker::PhantomData,
        }
    }
}

pub struct EffectIterator<
    'a,
    U: ColorContainer,
    T: ColorPointContainer<U>,
    E: Effect<U, T> + ?Sized,
> {
    effect: &'a mut E,
    phantom_color_value: std::marker::PhantomData<U>,
    phantom_color_point: std::marker::PhantomData<T>,
}

impl<'a, U: ColorContainer, T: ColorPointContainer<U>, E: Effect<U, T> + ?Sized> Iterator
    for EffectIterator<'a, U, T, E>
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.effect.get_frame())
    }
}
