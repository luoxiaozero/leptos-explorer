mod folding_range;

use napi_derive::napi;

#[napi]
pub fn folding_range(path: String) -> Vec<u32> {
    folding_range::folding_range(path)
        .into_iter()
        .flat_map(|r| [r.start as u32, r.end as u32])
        .collect()
}
