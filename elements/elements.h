#pragma once

#include "dsp/patch.h"

extern "C" {
  void Init(bool application);
  elements::Patch *GetPatch();
  void Pause(bool pause);
  void SetGate(bool newGate);
  void Elements_DMA1_Stream5_IRQHandler(void);
}
