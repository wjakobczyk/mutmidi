// Copyright 2014 Emilie Gillet.
// 
// Author: Emilie Gillet (emilie.o.gillet@gmail.com)
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
// 
// See http://creativecommons.org/licenses/MIT/ for more information.

#include <inttypes.h>

#include "elements/drivers/cv_adc.h"
#include "elements/drivers/codec.h"
#include "elements/drivers/debug_pin.h"
#include "elements/drivers/debug_port.h"
#include "elements/drivers/pots_adc.h"
#include "elements/drivers/system.h"
#include "elements/dsp/part.h"
#include "elements/cv_scaler.h"
#include "elements/ui.h"

#include "stmlib/stmlib.h"
#include "stmlib/dsp/dsp.h"

#include "stm32f4xx.h"
#include "stm32f4xx_gpio.h"
#include "stm32f4xx_rcc.h"

// #define PROFILE_INTERRUPT 1

using namespace elements;
using namespace stmlib;

Codec codec;
CvScaler cv_scaler;
DebugPort debug_port;
Part part;
Ui ui;

uint16_t reverb_buffer[32768] __attribute__ ((section (".ccmdata")));

// Default interrupt handlers.
extern "C" {

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wunused-but-set-variable"
void prvGetRegistersFromStack( uint32_t *pulFaultStackAddress )
{
/* These are volatile to try and prevent the compiler/linker optimising them
away as the variables never actually get used.  If the debugger won't show the
values of the variables, make them global my moving their declaration outside
of this function. */
volatile uint32_t r0;
volatile uint32_t r1;
volatile uint32_t r2;
volatile uint32_t r3;
volatile uint32_t r12;
volatile uint32_t lr; /* Link register. */
volatile uint32_t pc; /* Program counter. */
volatile uint32_t psr;/* Program status register. */
volatile uint32_t bfar;
volatile uint32_t cfsr;
volatile uint32_t hfsr;
volatile uint32_t dfsr;
volatile uint32_t afsr;
volatile uint32_t shcsr;

    r0 = pulFaultStackAddress[ 0 ];
    r1 = pulFaultStackAddress[ 1 ];
    r2 = pulFaultStackAddress[ 2 ];
    r3 = pulFaultStackAddress[ 3 ];

    r12 = pulFaultStackAddress[ 4 ];
    lr = pulFaultStackAddress[ 5 ];
    pc = pulFaultStackAddress[ 6 ];
    psr = pulFaultStackAddress[ 7 ];

    bfar = *((unsigned int*)0xE000ED38);
    cfsr = *((unsigned int*)0xE000ED28);
    hfsr = *((unsigned int*)0xE000ED2C);
    dfsr = *((unsigned int*)0xE000ED30);
    afsr = *((unsigned int*)0xE000ED3C);
    shcsr = SCB->SHCSR;

    /* When the following line is hit, the variables contain the register values. */
    for( ;; );
}
#pragma GCC diagnostic ignored "-Wunused-function"

static void HardFault_Handler() __attribute__( ( naked ) );
void NMI_Handler() { }
void HardFault_Handler()
{
    __asm volatile
    (
        " tst lr, #4                                                \n"
        " ite eq                                                    \n"
        " mrseq r0, msp                                             \n"
        " mrsne r0, psp                                             \n"
        " ldr r1, [r0, #24]                                         \n"
        " ldr r2, handler2_address_const                            \n"
        " bx r2                                                     \n"
        " handler2_address_const: .word prvGetRegistersFromStack    \n"
    );
}
#pragma GCC diagnostic pop
void MemManage_Handler() { while (1); }
void BusFault_Handler() { HardFault_Handler(); }
void UsageFault_Handler() { while (1); }
void SVC_Handler() { }
void DebugMon_Handler() { }
void PendSV_Handler() { }
void SysTick_Handler() {
  ui.Poll();
  if (debug_port.readable()) {
    uint8_t command = debug_port.Read();
    uint8_t response = ui.HandleFactoryTestingRequest(command);
    debug_port.Write(response);
  }
}

}

float blow_in[kAudioChunkSize];
float strike_in[kAudioChunkSize];
float out[kAudioChunkSize];
float aux[kAudioChunkSize];

const float kNoiseGateThreshold = 0.0001f;
float strike_in_level = 0.0f;
float blow_in_level = 0.0f;

void FillBuffer(Codec::Frame* input, Codec::Frame* output, size_t n) {
#ifdef PROFILE_INTERRUPT
  TIC
#endif  // PROFILE_INTERRUPT
  PerformanceState s;
  //cv_scaler.Read(part.mutable_patch(), &s);
  s.gate |= true;//ui.gate();
  s.note = 50;
  s.modulation = 0;
  s.strength = 1;
  for (size_t i = 0; i < n; ++i) {
    float blow_in_sample = static_cast<float>(input[i].r) / 32768.0f;
    float strike_in_sample = static_cast<float>(input[i].l) / 32768.0f;

    float error, gain;
    error = strike_in_sample * strike_in_sample - strike_in_level;
    strike_in_level += error * (error > 0.0f ? 0.1f : 0.0001f);
    gain = strike_in_level <= kNoiseGateThreshold 
          ? (1.0f / kNoiseGateThreshold) * strike_in_level : 1.0f;
    strike_in[i] = gain * strike_in_sample;
    
    error = blow_in_sample * blow_in_sample - blow_in_level;
    blow_in_level += error * (error > 0.0f ? 0.1f : 0.0001f);
    gain = blow_in_level <= kNoiseGateThreshold 
          ? (1.0f / kNoiseGateThreshold) * blow_in_level : 1.0f;
    blow_in[i] = gain * blow_in_sample;
  }
  part.Process(s, blow_in, strike_in, out, aux, n);
  for (size_t i = 0; i < n; ++i) {
    output[i].r = SoftConvert(out[i]);
    output[i].l = SoftConvert(aux[i]);
  }
#ifdef PROFILE_INTERRUPT
  TOC
#endif  // PROFILE_INTERRUPT
}

void Init() {
  System sys;
  
  sys.Init(true);

  // Init and seed the random parameters and generators with the serial number.
  part.Init(reverb_buffer);
  part.Seed((uint32_t*)(0x7a10), 3);

  cv_scaler.Init();
  ui.Init(&part, &cv_scaler);
  
  if (!codec.Init(32000, CODEC_PROTOCOL_PHILIPS, CODEC_FORMAT_16_BIT)) {
    //ui.Panic();
    while(1);
  }

  if (!codec.Start(&FillBuffer)) {
    //ui.Panic();
    while(1);
  }
  /*
  if (cv_scaler.freshly_baked()) {
#ifdef PROFILE_INTERRUPT
    DebugPin::Init();
#else
    debug_port.Init();
#endif  // PROFILE_INTERRUPT
  }
  */
  sys.StartTimers();
}

void delay(int time)
{
  volatile int i;
  for (i = 0; i < time * 4000; i++) {}
}

int main(void) {

    Init();
    
  GPIO_InitTypeDef gpio;

  RCC_AHB1PeriphClockCmd(RCC_AHB1Periph_GPIOD, ENABLE); // >TODO check

  GPIO_StructInit(&gpio);
  gpio.GPIO_Pin = GPIO_Pin_15;
  gpio.GPIO_Mode = GPIO_Mode_OUT;
  gpio.GPIO_OType = GPIO_OType_PP;
  gpio.GPIO_Speed = GPIO_Speed_2MHz;
  gpio.GPIO_PuPd = GPIO_PuPd_NOPULL;
  GPIO_Init(GPIOD, &gpio);

  while (1) {
      GPIO_SetBits(GPIOD, GPIO_Pin_15); // zapalenie diody
      delay(100);
      GPIO_ResetBits(GPIOD, GPIO_Pin_15); // zgaszenie diody
      delay(400);
  }

  while (1) {
    //ui.DoEvents();
  }
  return 0;
}
