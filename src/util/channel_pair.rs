use bevy_ecs::{
    component::Tick,
    system::{
        SystemParam,
        SystemMeta
    },
    world::{
        World,
        unsafe_world_cell::UnsafeWorldCell
    }
};
use std::sync::mpmc;


pub struct ChannelPair<'state, T>
where
    T : Send + Sync + 'static
{
    sender   : &'state mpmc::Sender<T>,
    receiver : &'state mpmc::Receiver<T>
}

impl<T> ChannelPair<'_, T>
where
    T : Send + Sync + 'static
{

    #[inline(always)]
    pub fn sender(&self) -> &mpmc::Sender<T> { &self.sender }

    #[inline(always)]
    pub fn receiver(&self) -> &mpmc::Receiver<T> { &self.receiver }

}


unsafe impl<T> SystemParam for ChannelPair<'_, T>
where
    T : Send + Sync + 'static
{
    type State                = (mpmc::Sender<T>, mpmc::Receiver<T>,);
    type Item<'world, 'state> = ChannelPair<'state, T>;

    #[inline(always)]
    fn init_state(_ : &mut World, _ : &mut SystemMeta) -> Self::State {
        mpmc::channel()
    }

    #[inline(always)]
    unsafe fn get_param<'world, 'state>(
        (sender, receiver,) : &'state mut Self::State,
        _                   : &SystemMeta,
        _                   : UnsafeWorldCell<'world>,
        _                   : Tick,
    ) -> Self::Item<'world, 'state> {
        ChannelPair { sender, receiver }
    }

}
