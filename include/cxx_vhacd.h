#pragma once
#include "rust/cxx.h"
#include <memory>

struct VHACDWrapperParams;

class VHACDWrapper {
public:
  VHACDWrapper();
  ~VHACDWrapper();
  uint32_t compute(rust::Slice<const float> points, rust::Slice<const uint32_t> tris, const VHACDWrapperParams& wrapper_params) const;
  void get_hull(uint32_t i, rust::Vec<float>& points, rust::Vec<uint32_t>& tris);

private:
  void* m_pIVHACD;
};

std::unique_ptr<VHACDWrapper> new_VHACDWrapper();
