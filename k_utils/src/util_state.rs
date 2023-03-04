use std::{fmt::Debug, hash::Hash};

pub trait StateContraint:
    Clone + Copy + Eq + Default + PartialEq + Debug + Hash + Send + Sync + 'static
{
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum UtilState<PluginType: StateContraint> {
    #[default]
    Uninitialized,
    Enter,
    Running,
    Paused,
    _Unreachable(std::marker::PhantomData<&'static PluginType>),
}
