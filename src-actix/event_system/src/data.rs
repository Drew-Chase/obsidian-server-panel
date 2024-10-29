pub struct EventManager<S>
where
    S: Sized + Clone,
{
    pub data: Vec<S>,
    pub subscribers: Vec<Subscriber<S>>,
}

pub struct Subscriber<S>
where
    S: Sized + Clone,
{
    pub(crate) id: u32,
    pub(crate) callback: Box<dyn Fn(&Self, S) + 'static + Send + Sync>,
}
