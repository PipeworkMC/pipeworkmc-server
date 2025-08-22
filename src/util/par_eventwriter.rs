use bevy_ecs::{
    component::Tick,
    event::Event,
    system::{
        SystemParam,
        SystemMeta
    },
    world::{
        World,
        unsafe_world_cell::UnsafeWorldCell
    }
};
use core::iter;
use std::sync::mpmc;


pub struct ParallelEventWriter<E>
where
    E : Event
{
    sender : mpmc::Sender<E>
}

impl<E> ParallelEventWriter<E>
where
    E : Event
{

    #[inline]
    pub fn write(&self, event : E) {
        _ = self.sender.send(event);
    }

    #[inline]
    pub fn write_batch(&self, events : impl IntoIterator<Item = E>) {
        for event in events {
            _ = self.sender.send(event);
        }
    }

    #[inline(always)]
    pub fn write_default(&self)
    where
        E : Default
    { self.write(E::default()) }

}


unsafe impl<E> SystemParam for ParallelEventWriter<E>
where
    E : Event
{
    type State                = (mpmc::Sender<E>, mpmc::Receiver<E>,);
    type Item<'world, 'state> = ParallelEventWriter<E>;

    fn init_state(_ : &mut World, _ : &mut SystemMeta) -> Self::State {
        mpmc::channel()
    }

    unsafe fn get_param<'world, 'state>(
        (sender, _,) : &'state mut Self::State,
        _            : &SystemMeta,
        _            : UnsafeWorldCell<'world>,
        _            : Tick,
    ) -> Self::Item<'world, 'state> {
        ParallelEventWriter { sender : sender.clone() }
    }

    fn apply(
        (_, receiver,) : &mut Self::State,
        _              : &SystemMeta,
        world          : &mut World
    ) {
        world.send_event_batch(iter::from_fn(|| receiver.try_recv().ok()));
    }

}
