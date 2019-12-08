#pragma once

#include "dsp/patch.h"

extern "C" {
  void Elements_Init(bool application);
  elements::Patch *Elements_GetPatch();
  void Elements_Pause(bool pause);
  void Elements_SetGate(bool newGate);
  void Elements_SetNote(float newNote);
  void Elements_SetStrength(float newStrength);
  void Elements_SetModulation(float newModulation);
  void Elements_DMA1_Stream5_IRQHandler(void);
}
