use crate::SegmentDescription;

#[derive(Debug, Clone, Copy)]
pub struct SkeletonRigDescription {
    pub segment_chain_constraint: f32,
    pub segments: &'static [SkeletonSegmentDescription],
}

#[derive(Debug, Clone, Copy)]
pub struct SkeletonSegmentDescription {
    pub segment: SegmentDescription,
    pub childs: &'static [SkeletonSegmentDescription],
}
