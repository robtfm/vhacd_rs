#include "vhacd_rs/include/cxx_vhacd.h"
#include "vhacd_rs/src/lib.rs.h"

#define ENABLE_VHACD_IMPLEMENTATION 1
#define VHACD_DISABLE_THREADING 1
#include "vhacd_rs/include/VHACD.h"

VHACDWrapper::VHACDWrapper() {
  m_pIVHACD = VHACD::CreateVHACD();
}

VHACDWrapper::~VHACDWrapper() {
  VHACD::IVHACD* p = static_cast<VHACD::IVHACD*>(m_pIVHACD);
  p->Clean();
  p->Release();
}

std::unique_ptr<VHACDWrapper> new_VHACDWrapper() {
  return std::unique_ptr<VHACDWrapper>(new VHACDWrapper());
}

uint32_t VHACDWrapper::compute(rust::Slice<const float> points, rust::Slice<const uint32_t> tris, const VHACDWrapperParams& wrapper_params) const {
  VHACD::IVHACD* p = static_cast<VHACD::IVHACD*>(m_pIVHACD);
  VHACD::IVHACD::Parameters params{};
  params.m_asyncACD = false;
  params.m_resolution = wrapper_params.resolution;
  params.m_maxConvexHulls = wrapper_params.max_hulls;
  params.m_maxRecursionDepth = wrapper_params.depth;
  params.m_minimumVolumePercentErrorAllowed = wrapper_params.error * 100.0;

  p->Compute(points.data(), points.length() / 3, tris.data(), tris.length() / 3, params);
  uint32_t hulls = p->GetNConvexHulls();
  return hulls;
}

void VHACDWrapper::get_hull(uint32_t i, rust::Vec<float>& points, rust::Vec<uint32_t>& tris) {
  VHACD::IVHACD* p = static_cast<VHACD::IVHACD*>(m_pIVHACD);
  VHACD::IVHACD::ConvexHull hull;
  p->GetConvexHull(i, hull);

  std::vector<VHACD::Vertex>::iterator iter_points = hull.m_points.begin();
  for (iter_points; iter_points < hull.m_points.end(); iter_points++) {
    points.push_back(iter_points->mX);
    points.push_back(iter_points->mY);
    points.push_back(iter_points->mZ);
  }

  std::vector<VHACD::Triangle>::iterator iter_tris = hull.m_triangles.begin();
  for (iter_tris; iter_tris < hull.m_triangles.end(); iter_tris++) {
    tris.push_back((float) iter_tris->mI0);
    tris.push_back((float) iter_tris->mI1);
    tris.push_back((float) iter_tris->mI2);
  }
}