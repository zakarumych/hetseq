
use shred::{ResourceId, Resources, SystemData};

use List;

impl<'a, S> SystemData<'a> for List<S>
    where S: SystemData<'a>
{
    fn fetch(res: &'a Resources, id: usize) -> Self {
        List(S::fetch(res, id))
    }
    fn reads(id: usize) -> Vec<ResourceId> { S::reads(id) }
    fn writes(id: usize) -> Vec<ResourceId> { S::writes(id) }
}