//! 命題論理の基本構造と演繹規則を提供するモジュールにゃん。
/// # Examples
///
/// ```
/// pub struct P;
///impl prop::Prop for P {}
///fn p_naraba_p() -> Cond<P, P> {
///let _1 = apl::a1();
///let _2 = apl::a2();
///let _3 = (apl::mp())(_1, _2);
///let _4: Cond<P, Cond<P, P>> = apl::a1();
///(apl::mp())(_4, _3)
///}
/// ```
pub use prop::*;
pub mod apl {
    use super::prop::*;
    ///A → (B → A)
    pub fn a1<A, B>() -> Cond<A, Cond<B, A>>
    where
        A: Prop,
        B: Prop,
    {
        unreachable!()
    }
    ///(A → (B → C)) → ((A → B) → (A → C))
    pub fn a2<A, B, C>() -> Cond<Cond<A, Cond<B, C>>, Cond<Cond<A, B>, Cond<A, C>>>
    where
        A: Prop,
        B: Prop,
        C: Prop,
    {
        unreachable!()
    }
    ///(¬B → ¬A) → ((¬B → A) → B
    pub fn a3<A, B>() -> Cond<Cond<Neg<B>, Neg<A>>, Cond<Cond<Neg<B>, A>, B>>
    where
        A: Prop,
        B: Prop,
    {
        unreachable!()
    }
    ///A,A→B⊢B
    pub fn mp<A, B>() -> Box<dyn Fn(A, Cond<A, B>) -> B>
    where
        A: Prop,
        B: Prop,
    {
        unreachable!()
    }
}
pub mod prop {
    pub trait Prop {}
    ///￢T
    pub struct Neg<T>(std::marker::PhantomData<T>)
    where
        T: Prop;
    impl<T:Prop> Prop for Neg<T> {}
    ///T→U
    pub struct Cond<T, U>(std::marker::PhantomData<T>, std::marker::PhantomData<U>)where
    T: Prop,
    U:Prop;
    impl<T:Prop, U:Prop> Prop for Cond<T, U> {}
}
