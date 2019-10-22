#pragma once

#include "dsp/patch.h"

extern "C" {
  void Init(bool application);
  elements::Patch *GetPatch();
  void SetGate(int newGate);
  void Elements_DMA1_Stream5_IRQHandler(void);
}
