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


/// A parallel event writer.
///
/// Unlike [`EventWriter`](bevy_ecs::event::EventWriter), this can be used in with
///  [`EventReader::par_read`](bevy_ecs::event::EventReader::par_read),
///  [`Query::par_iter`](bevy_ecs::system::Query::par_iter),
///  etc.
pub struct ParallelEventWriter<'state, E>
where
    E : Event
{
    sender  : &'state mpmc::Sender<E>
}

impl<E> ParallelEventWriter<'_, E>
where
    E : Event
{

    /// Send an event, which can later be read by [`EventReader`](bevy_ecs::event::EventReader)s.
    #[inline]
    pub fn write(&self, event : E) {
        _ = self.sender.send(event);
    }

    /// Sends multiple events all at once, which can later be read by [`EventReader`](bevy_ecs::event::EventReader)s.
    #[inline]
    pub fn write_batch(&self, events : impl IntoIterator<Item = E>) {
        for event in events {
            _ = self.sender.send(event);
        }
    }

    /// Sends the default value of the event. Useful when the event is an empty struct.
    #[inline]
    pub fn write_default(&self)
    where
        E : Default
    { self.write(E::default()) }

}


unsafe impl<E> SystemParam for ParallelEventWriter<'_, E>
where
    E : Event
{
    type State                = (mpmc::Sender<E>, mpmc::Receiver<E>,);
    type Item<'world, 'state> = ParallelEventWriter<'state, E>;

    #[inline]
    fn init_state(_ : &mut World, _ : &mut SystemMeta) -> Self::State {
        mpmc::channel()
    }

    #[inline]
    unsafe fn get_param<'world, 'state>(
        (sender, _,) : &'state mut Self::State,
        _            : &SystemMeta,
        _            : UnsafeWorldCell<'world>,
        _            : Tick,
    ) -> Self::Item<'world, 'state> {
        ParallelEventWriter { sender }
    }

    fn apply(
        (_, receiver,) : &mut Self::State,
        _              : &SystemMeta,
        world          : &mut World
    ) {
        world.send_event_batch(iter::from_fn(|| receiver.try_recv().ok()));
    }

}
