#[cxx::bridge]
mod ffi {
    pub struct VHACDWrapperParams {
        resolution: u32,
        error: f32,
        max_hulls: u32,
        depth: u32,
    }

    unsafe extern "C++" {
        include!("vhacd_rs/include/cxx_vhacd.h");

        type VHACDWrapper;

        fn new_VHACDWrapper() -> UniquePtr<VHACDWrapper>;
        fn compute(&self, points: &[f32], indices: &[u32], params: &VHACDWrapperParams) -> u32;
        fn get_hull(
            self: Pin<&mut VHACDWrapper>,
            i: u32,
            points: &mut Vec<f32>,
            indices: &mut Vec<u32>,
        );
    }
}

#[derive(Default)]
pub struct ConvexHull {
    pub points: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
    pub min_bound: [f32; 3],
    pub max_bound: [f32; 3],
}

struct Hulls<'a> {
    parent: &'a mut VHACD,
    index: u32,
}

impl<'a> ExactSizeIterator for Hulls<'a> {
    fn len(&self) -> usize {
        (self.parent.count - self.index) as usize
    }
}

impl<'a> Iterator for Hulls<'a> {
    type Item = ConvexHull;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.parent.count {
            return None;
        }

        let mut points = Vec::default();
        let mut ch = ConvexHull::default();
        let mut min_bound = [f32::MAX; 3];
        let mut max_bound = [f32::MIN; 3];

        self.parent
            .wrapper
            .pin_mut()
            .get_hull(self.index, &mut points, &mut ch.indices);
        ch.points.extend(points.chunks_exact(3).map(|chunk| {
            let point: [f32; 3] = chunk.try_into().unwrap();
            for i in 0..3 {
                min_bound[i] = min_bound[i].min(point[i]);
                max_bound[i] = max_bound[i].max(point[i]);
            }
            point
        }));

        ch.min_bound = min_bound;
        ch.max_bound = max_bound;

        self.index += 1;
        Some(ch)
    }
}

pub use ffi::VHACDWrapperParams;

pub struct VHACD {
    wrapper: cxx::UniquePtr<ffi::VHACDWrapper>,
    count: u32,
}

impl VHACD {
    pub fn new() -> Self {
        Self {
            wrapper: ffi::new_VHACDWrapper(),
            count: 0,
        }
    }

    pub fn compute(
        &mut self,
        points: &[[f32; 3]],
        indices: &[u32],
        params: &VHACDWrapperParams,
    ) -> impl ExactSizeIterator<Item = ConvexHull> + '_ {
        let points: Vec<f32> = points
            .into_iter()
            .map(|xyz| [xyz[0], xyz[1], xyz[2]])
            .flatten()
            .collect();
        self.count = self.wrapper.compute(&points, indices, params);
        Hulls {
            parent: self,
            index: 0,
        }
    }
}

#[test]
fn test() {
    let mut vhacd = VHACD::new();

    let points = vec![[0.0, 0.0, 0.0], [1.0, 1.0, 0.0], [1.0, 0.0, 0.0]];
    let indices = vec![0, 1, 2];

    let params = VHACDWrapperParams {
        resolution: 100000,
        error: 0.04,
        max_hulls: 1024,
        depth: 10,
    };

    let hulls = vhacd.compute(&points, &indices, &params);

    assert_eq!(hulls.len(), 1);

    for hull in hulls {
        assert!(!hull.points.is_empty() && !hull.indices.is_empty());
    }
}
