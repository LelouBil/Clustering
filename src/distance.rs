use crate::file_loading::Instance;

pub(crate) fn jaccard(a: &Instance, b: &Instance) -> f32 {
    1.0 - ((a.proprietes.intersection(&b.proprietes).count() as f32) / (a.proprietes.union(&b.proprietes).count() as f32))
}