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
//
// -----------------------------------------------------------------------------
//
// WM8371 Codec support.

#include "elements/drivers/codec.h"

#include <string.h>

#define DAC_CS43L22			//http://www.mind-dump.net/configuring-the-stm32f4-discovery-for-audio
//#define DAC_WM8731

#ifdef DAC_WM8731
#define CODEC_I2C                      I2C2
#define CODEC_I2C_CLK                  RCC_APB1Periph_I2C2
#define CODEC_I2C_GPIO_CLOCK           RCC_AHB1Periph_GPIOB
#define CODEC_I2C_GPIO_AF              GPIO_AF_I2C2
#define CODEC_I2C_GPIO                 GPIOB
#define CODEC_I2C_SCL_PIN              GPIO_Pin_10
#define CODEC_I2C_SDA_PIN              GPIO_Pin_11
#define CODEC_I2C_SCL_PINSRC           GPIO_PinSource10
#define CODEC_I2C_SDA_PINSRC           GPIO_PinSource11
#define CODEC_TIMEOUT                  ((uint32_t)0x1000)
#define CODEC_LONG_TIMEOUT             ((uint32_t)(300 * CODEC_TIMEOUT))
#define CODEC_I2C_SPEED                100000

#define CODEC_I2S                      SPI2
#define CODEC_I2S_EXT                  I2S2ext
#define CODEC_I2S_CLK                  RCC_APB1Periph_SPI2
#define CODEC_I2S_ADDRESS              0x4000380C
#define CODEC_I2S_EXT_ADDRESS          0x4000340C
#define CODEC_I2S_GPIO_AF              GPIO_AF_SPI2
#define CODEC_I2S_IRQ                  SPI2_IRQn
#define CODEC_I2S_EXT_IRQ              SPI2_IRQn
#define CODEC_I2S_GPIO_CLOCK           (RCC_AHB1Periph_GPIOC | RCC_AHB1Periph_GPIOB)
#define CODEC_I2S_WS_PIN               GPIO_Pin_12
#define CODEC_I2S_SCK_PIN              GPIO_Pin_13
#define CODEC_I2S_SDI_PIN              GPIO_Pin_14
#define CODEC_I2S_SDO_PIN              GPIO_Pin_15
#define CODEC_I2S_MCK_PIN              GPIO_Pin_6
#define CODEC_I2S_WS_PINSRC            GPIO_PinSource12
#define CODEC_I2S_SCK_PINSRC           GPIO_PinSource13
#define CODEC_I2S_SDI_PINSRC           GPIO_PinSource14
#define CODEC_I2S_SDO_PINSRC           GPIO_PinSource15
#define CODEC_I2S_MCK_PINSRC           GPIO_PinSource6
#define CODEC_I2S_GPIO                 GPIOB
#define CODEC_I2S_MCK_GPIO             GPIOC
#define CODEC_I2S_WS_GPIO              GPIOB
#define AUDIO_I2S_IRQHandler           SPI2_IRQHandler

#define AUDIO_DMA_PERIPH_DATA_SIZE     DMA_PeripheralDataSize_HalfWord
#define AUDIO_DMA_MEM_DATA_SIZE        DMA_MemoryDataSize_HalfWord
#define AUDIO_I2S_DMA_CLOCK            RCC_AHB1Periph_DMA1
#define AUDIO_I2S_DMA_STREAM           DMA1_Stream4
#define AUDIO_I2S_DMA_DREG             CODEC_I2S_ADDRESS
#define AUDIO_I2S_DMA_CHANNEL          DMA_Channel_0
#define AUDIO_I2S_DMA_IRQ              DMA1_Stream4_IRQn
#define AUDIO_I2S_DMA_FLAG_TC          DMA_FLAG_TCIF4
#define AUDIO_I2S_DMA_FLAG_HT          DMA_FLAG_HTIF4
#define AUDIO_I2S_DMA_FLAG_FE          DMA_FLAG_FEIF4
#define AUDIO_I2S_DMA_FLAG_TE          DMA_FLAG_TEIF4
#define AUDIO_I2S_DMA_FLAG_DME         DMA_FLAG_DMEIF4
#define AUDIO_I2S_EXT_DMA_STREAM       DMA1_Stream3
#define AUDIO_I2S_EXT_DMA_DREG         CODEC_I2S_EXT_ADDRESS
#define AUDIO_I2S_EXT_DMA_CHANNEL      DMA_Channel_3
#define AUDIO_I2S_EXT_DMA_IRQ          DMA1_Stream3_IRQn
#define AUDIO_I2S_EXT_DMA_FLAG_TC      DMA_FLAG_TCIF3
#define AUDIO_I2S_EXT_DMA_FLAG_HT      DMA_FLAG_HTIF3
#define AUDIO_I2S_EXT_DMA_FLAG_FE      DMA_FLAG_FEIF3
#define AUDIO_I2S_EXT_DMA_FLAG_TE      DMA_FLAG_TEIF3
#define AUDIO_I2S_EXT_DMA_FLAG_DME     DMA_FLAG_DMEIF3
#define AUDIO_I2S_EXT_DMA_REG          DMA1
#define AUDIO_I2S_EXT_DMA_ISR          LISR
#define AUDIO_I2S_EXT_DMA_IFCR         LIFCR

#define W8731_ADDR_0 0x1A
#define W8731_ADDR_1 0x1B
#define W8731_NUM_REGS 10
#define CODEC_ADDRESS           (W8731_ADDR_0 << 1)
#endif

#ifdef DAC_CS43L22
#define CODEC_I2C                      I2C1
#define CODEC_I2C_CLK                  RCC_APB1Periph_I2C1
#define CODEC_I2C_GPIO_CLOCK           RCC_AHB1Periph_GPIOB
#define CODEC_I2C_GPIO                 GPIOB
#define CODEC_I2C_SCL_PIN              GPIO_Pin_6
#define CODEC_I2C_SDA_PIN              GPIO_Pin_9
#define CODEC_I2C_SCL_PINSRC           GPIO_PinSource6
#define CODEC_I2C_SDA_PINSRC           GPIO_PinSource9
#define CODEC_I2C_GPIO_AF              GPIO_AF_I2C1
#define CODEC_TIMEOUT                  ((uint32_t)0x1000)
#define CODEC_LONG_TIMEOUT             ((uint32_t)(300 * CODEC_TIMEOUT))
#define CODEC_I2C_SPEED                100000

#define CODEC_RESET_GPIO			   GPIOD
#define CODEC_RESET_PIN				   GPIO_Pin_4

#define CODEC_I2S                      SPI3
#define CODEC_I2S_CLK                  RCC_APB1Periph_SPI3
#define CODEC_I2S_GPIO_CLOCK           (RCC_AHB1Periph_GPIOC | RCC_AHB1Periph_GPIOD | RCC_AHB1Periph_GPIOA)
#define CODEC_I2S_GPIO                 GPIOC
#define CODEC_I2S_MCK_GPIO             GPIOC
#define CODEC_I2S_WS_GPIO              GPIOA
#define CODEC_I2S_WS_PIN               GPIO_Pin_4
#define CODEC_I2S_SCK_PIN              GPIO_Pin_10
#define CODEC_I2S_SDI_PIN              GPIO_Pin_12
#define CODEC_I2S_SDO_PIN              GPIO_Pin_12
#define CODEC_I2S_MCK_PIN              GPIO_Pin_7
#define CODEC_I2S_WS_PINSRC            GPIO_PinSource4
#define CODEC_I2S_SCK_PINSRC           GPIO_PinSource10
#define CODEC_I2S_SDI_PINSRC           GPIO_PinSource12
#define CODEC_I2S_SDO_PINSRC           GPIO_PinSource12
#define CODEC_I2S_MCK_PINSRC           GPIO_PinSource7
#define CODEC_I2S_GPIO_AF              GPIO_AF_SPI3
#define CODEC_I2S_ADDRESS              0x40003C0C

#define CODEC_ADDRESS           	   0x94

#define AUDIO_I2S_DMA_DREG             CODEC_I2S_ADDRESS
#define AUDIO_I2S_DMA_CHANNEL          DMA_Channel_0
#define AUDIO_I2S_DMA_STREAM           DMA1_Stream5
#define AUDIO_I2S_DMA_CLOCK            RCC_AHB1Periph_DMA1
#define AUDIO_I2S_DMA_REG          		 DMA1
#define AUDIO_I2S_DMA_FLAG_TC      		 DMA_FLAG_TCIF5
#define AUDIO_I2S_DMA_FLAG_HT      		 DMA_FLAG_HTIF5
#define AUDIO_I2S_DMA_ISR          		 HISR
#define AUDIO_I2S_DMA_IFCR         		 HIFCR
#define AUDIO_I2S_DMA_IRQ               DMA1_Stream5_IRQn

#define CODEC_MAPBYTE_INC 0x80

//register map bytes for CS42L22 (see page 35)
#define CODEC_MAP_CHIP_ID 0x01
#define CODEC_MAP_PWR_CTRL1 0x02
#define CODEC_MAP_PWR_CTRL2 0x04
#define CODEC_MAP_CLK_CTRL  0x05
#define CODEC_MAP_IF_CTRL1  0x06
#define CODEC_MAP_IF_CTRL2  0x07
#define CODEC_MAP_PASSTHROUGH_A_SELECT 0x08
#define CODEC_MAP_PASSTHROUGH_B_SELECT 0x09
#define CODEC_MAP_ANALOG_SET 0x0A
#define CODEC_MAP_PASSTHROUGH_GANG_CTRL 0x0C
#define CODEC_MAP_PLAYBACK_CTRL1 0x0D
#define CODEC_MAP_MISC_CTRL 0x0E
#define CODEC_MAP_PLAYBACK_CTRL2 0x0F
#define CODEC_MAP_PASSTHROUGH_A_VOL 0x14
#define CODEC_MAP_PASSTHROUGH_B_VOL 0x15
#define CODEC_MAP_PCMA_VOL 0x1A
#define CODEC_MAP_PCMB_VOL 0x1B
#define CODEC_MAP_BEEP_FREQ_ONTIME 0x1C
#define CODEC_MAP_BEEP_VOL_OFFTIME 0x1D
#define CODEC_MAP_BEEP_TONE_CFG 0x1E
#define CODEC_MAP_TONE_CTRL 0x1F
#define CODEC_MAP_MASTER_A_VOL 0x20
#define CODEC_MAP_MASTER_B_VOL 0x21
#define CODEC_MAP_HP_A_VOL 0x22
#define CODEC_MAP_HP_B_VOL 0x23
#define CODEC_MAP_SPEAK_A_VOL 0x24
#define CODEC_MAP_SPEAK_B_VOL 0x25
#define CODEC_MAP_CH_MIX_SWAP 0x26
#define CODEC_MAP_LIMIT_CTRL1 0x27
#define CODEC_MAP_LIMIT_CTRL2 0x28
#define CODEC_MAP_LIMIT_ATTACK 0x29
#define CODEC_MAP_OVFL_CLK_STATUS 0x2E
#define CODEC_MAP_BATT_COMP 0x2F
#define CODEC_MAP_VP_BATT_LEVEL 0x30
#define CODEC_MAP_SPEAK_STATUS 0x31
#define CODEC_MAP_CHARGE_PUMP_FREQ 0x34

#endif

#define WAIT_LONG(x) { \
  uint32_t timeout = CODEC_LONG_TIMEOUT; \
  while (x) { if ((timeout--) == 0) return false; } \
}

#define WAIT(x) { \
  uint32_t timeout = CODEC_TIMEOUT; \
  while (x) { if ((timeout--) == 0) return false; } \
}

namespace elements {

/* static */
Codec* Codec::instance_;

enum CodecRegister {
#ifdef DAC_WM8731
  CODEC_REG_LEFT_LINE_IN = 0x00,
  CODEC_REG_RIGHT_LINE_IN = 0x01,
  CODEC_REG_LEFT_HEADPHONES_OUT = 0x02,
  CODEC_REG_RIGHT_HEADPHONES_OUT = 0x03,
  CODEC_REG_ANALOGUE_ROUTING = 0x04,
  CODEC_REG_DIGITAL_ROUTING = 0x05,
  CODEC_REG_POWER_MANAGEMENT = 0x06,
  CODEC_REG_DIGITAL_FORMAT = 0x07,
  CODEC_REG_SAMPLE_RATE = 0x08,
  CODEC_REG_ACTIVE = 0x09,
  CODEC_REG_RESET = 0x0f,
#endif
};

bool Codec::InitializeGPIO() {
  GPIO_InitTypeDef gpio_init;

  // Start GPIO peripheral clocks.
  RCC_AHB1PeriphClockCmd(CODEC_I2C_GPIO_CLOCK | CODEC_I2S_GPIO_CLOCK, ENABLE);

#ifdef CODEC_RESET_PIN
  gpio_init.GPIO_Pin = CODEC_RESET_PIN;
  gpio_init.GPIO_Mode = GPIO_Mode_OUT;
  gpio_init.GPIO_OType = GPIO_OType_PP;
  gpio_init.GPIO_PuPd = GPIO_PuPd_DOWN;
  gpio_init.GPIO_Speed = GPIO_Speed_50MHz;

  GPIO_Init(CODEC_RESET_GPIO, &gpio_init);
#endif

  // Initialize I2C pins
  gpio_init.GPIO_Pin = CODEC_I2C_SCL_PIN | CODEC_I2C_SDA_PIN; 
  gpio_init.GPIO_Mode = GPIO_Mode_AF;
  gpio_init.GPIO_Speed = GPIO_Speed_25MHz;
  gpio_init.GPIO_OType = GPIO_OType_OD;
  gpio_init.GPIO_PuPd  = GPIO_PuPd_NOPULL;
  GPIO_Init(CODEC_I2C_GPIO, &gpio_init);

  // Connect pins to I2C peripheral
  GPIO_PinAFConfig(CODEC_I2C_GPIO, CODEC_I2C_SCL_PINSRC, CODEC_I2C_GPIO_AF);
  GPIO_PinAFConfig(CODEC_I2C_GPIO, CODEC_I2C_SDA_PINSRC, CODEC_I2C_GPIO_AF);
  
  // Initialize I2S pins
  gpio_init.GPIO_Pin = CODEC_I2S_SCK_PIN | CODEC_I2S_SDO_PIN | \
      CODEC_I2S_SDI_PIN;
  gpio_init.GPIO_Mode = GPIO_Mode_AF;
  gpio_init.GPIO_Speed = GPIO_Speed_50MHz;
  gpio_init.GPIO_OType = GPIO_OType_PP;
  gpio_init.GPIO_PuPd = GPIO_PuPd_NOPULL;
  GPIO_Init(CODEC_I2S_GPIO, &gpio_init);
  
  gpio_init.GPIO_Pin = CODEC_I2S_MCK_PIN; 
  GPIO_Init(CODEC_I2S_MCK_GPIO, &gpio_init);

  gpio_init.GPIO_Pin = CODEC_I2S_WS_PIN;
  GPIO_Init(CODEC_I2S_WS_GPIO, &gpio_init);

  // Connect pins to I2S peripheral.
  GPIO_PinAFConfig(CODEC_I2S_WS_GPIO, CODEC_I2S_WS_PINSRC, CODEC_I2S_GPIO_AF);
  GPIO_PinAFConfig(CODEC_I2S_GPIO, CODEC_I2S_SCK_PINSRC, CODEC_I2S_GPIO_AF);
  GPIO_PinAFConfig(CODEC_I2S_GPIO, CODEC_I2S_SDO_PINSRC, CODEC_I2S_GPIO_AF);
  GPIO_PinAFConfig(CODEC_I2S_GPIO, CODEC_I2S_SDI_PINSRC, CODEC_I2S_GPIO_AF);
  GPIO_PinAFConfig(CODEC_I2S_MCK_GPIO, CODEC_I2S_MCK_PINSRC, CODEC_I2S_GPIO_AF); 
  return true;
}

bool Codec::InitializeControlInterface() {
  I2C_InitTypeDef i2c_init;

  // Initialize I2C
  RCC_APB1PeriphClockCmd(CODEC_I2C_CLK, ENABLE);

  I2C_DeInit(CODEC_I2C);
  i2c_init.I2C_Mode = I2C_Mode_I2C;
  i2c_init.I2C_DutyCycle = I2C_DutyCycle_2;
  i2c_init.I2C_OwnAddress1 = 0x33;
  i2c_init.I2C_Ack = I2C_Ack_Enable;
  i2c_init.I2C_AcknowledgedAddress = I2C_AcknowledgedAddress_7bit;
  i2c_init.I2C_ClockSpeed = CODEC_I2C_SPEED;
  
  I2C_Init(CODEC_I2C, &i2c_init);
  I2C_Cmd(CODEC_I2C, ENABLE);  

  return true;
}

bool Codec::InitializeAudioInterface(
    uint32_t sample_rate,
    CodecProtocol protocol,
    CodecFormat format) {

  // Configure PLL and I2S master clock.
  RCC_I2SCLKConfig(RCC_I2S2CLKSource_PLLI2S);
  
  // The following values have been computed for a 8Mhz external crystal!
  RCC_PLLI2SCmd(DISABLE);
  if (sample_rate == 48000) {
    // 47.992kHz
    RCC_PLLI2SConfig(258, 3);
  } else if (sample_rate == 44100) {
    // 44.11kHz
    RCC_PLLI2SConfig(271, 6);
  } else if (sample_rate == 32000) {
    // 32.003kHz
    RCC_PLLI2SConfig(426, 4);
  } else if (sample_rate == 96000) {
    // 95.95 kHz
    RCC_PLLI2SConfig(393, 4);
  } else {
    // Unsupported sample rate!
    return false;
  }
  RCC_PLLI2SCmd(ENABLE);
  WAIT(RCC_GetFlagStatus(RCC_FLAG_PLLI2SRDY) == RESET);

  RCC_APB1PeriphClockCmd(CODEC_I2S_CLK, ENABLE);

  // Initialize I2S
  I2S_InitTypeDef i2s_init;
  
  SPI_I2S_DeInit(CODEC_I2S);
  i2s_init.I2S_AudioFreq = sample_rate;
  i2s_init.I2S_Standard = protocol;
  i2s_init.I2S_DataFormat = format;
  i2s_init.I2S_CPOL = I2S_CPOL_Low;
  i2s_init.I2S_Mode = I2S_Mode_MasterTx;
  i2s_init.I2S_MCLKOutput = I2S_MCLKOutput_Enable;

  // Initialize the I2S main channel for TX
  I2S_Init(CODEC_I2S, &i2s_init);
  
#ifdef CODEC_I2S_EXT
  // Initialize the I2S extended channel for RX
  I2S_FullDuplexConfig(CODEC_I2S_EXT, &i2s_init);
#endif

  return true;
}

bool Codec::WriteControlRegister16(uint8_t address, uint16_t data) {
	return WriteControlRegister((uint8_t)((address << 1) & 0xfe) | ((data >> 8) & 0x01), (uint8_t)(data & 0xff));
}

bool Codec::WriteControlRegister(uint8_t byte_1, uint8_t byte_2) {
  WAIT_LONG(I2C_GetFlagStatus(CODEC_I2C, I2C_FLAG_BUSY));
  
  I2C_GenerateSTART(CODEC_I2C, ENABLE);
  WAIT(!I2C_CheckEvent(CODEC_I2C, I2C_EVENT_MASTER_MODE_SELECT));

  I2C_Send7bitAddress(CODEC_I2C, CODEC_ADDRESS, I2C_Direction_Transmitter);
  WAIT(!I2C_CheckEvent(CODEC_I2C, I2C_EVENT_MASTER_TRANSMITTER_MODE_SELECTED));

  I2C_SendData(CODEC_I2C, byte_1);
  WAIT(!I2C_CheckEvent(CODEC_I2C, I2C_EVENT_MASTER_BYTE_TRANSMITTING));

  I2C_SendData(CODEC_I2C, byte_2);
  WAIT(!I2C_CheckEvent(CODEC_I2C, I2C_EVENT_MASTER_BYTE_TRANSMITTING));

  WAIT_LONG(!I2C_GetFlagStatus(CODEC_I2C, I2C_FLAG_BTF));

  I2C_GenerateSTOP(CODEC_I2C, ENABLE);  

  return true;  
}

bool Codec::WriteControlRegister(uint8_t byte_1, uint8_t byte_2, uint8_t byte_3) {

  WAIT_LONG(I2C_GetFlagStatus(CODEC_I2C, I2C_FLAG_BUSY));

  I2C_GenerateSTART(CODEC_I2C, ENABLE);
  WAIT(!I2C_CheckEvent(CODEC_I2C, I2C_EVENT_MASTER_MODE_SELECT));

  I2C_Send7bitAddress(CODEC_I2C, CODEC_ADDRESS, I2C_Direction_Transmitter);
  WAIT(!I2C_CheckEvent(CODEC_I2C, I2C_EVENT_MASTER_TRANSMITTER_MODE_SELECTED));

  I2C_SendData(CODEC_I2C, byte_1);
  WAIT(!I2C_CheckEvent(CODEC_I2C, I2C_EVENT_MASTER_BYTE_TRANSMITTING));

  I2C_SendData(CODEC_I2C, byte_2);
  WAIT(!I2C_CheckEvent(CODEC_I2C, I2C_EVENT_MASTER_BYTE_TRANSMITTING));

  I2C_SendData(CODEC_I2C, byte_3);
  WAIT(!I2C_CheckEvent(CODEC_I2C, I2C_EVENT_MASTER_BYTE_TRANSMITTING));

  WAIT_LONG(!I2C_GetFlagStatus(CODEC_I2C, I2C_FLAG_BTF));

  I2C_GenerateSTOP(CODEC_I2C, ENABLE);

  return true;
}

uint8_t Codec::ReadControlRegister(uint8_t mapbyte) {
	uint8_t receivedByte = 0;

  WAIT_LONG(I2C_GetFlagStatus(CODEC_I2C, I2C_FLAG_BUSY));

  I2C_GenerateSTART(CODEC_I2C, ENABLE);
  WAIT(!I2C_CheckEvent(CODEC_I2C, I2C_EVENT_MASTER_MODE_SELECT));

  I2C_Send7bitAddress(CODEC_I2C, CODEC_ADDRESS, I2C_Direction_Transmitter);
  WAIT(!I2C_CheckEvent(CODEC_I2C, I2C_EVENT_MASTER_TRANSMITTER_MODE_SELECTED));

  I2C_SendData(CODEC_I2C, mapbyte);
  WAIT(!I2C_CheckEvent(CODEC_I2C, I2C_EVENT_MASTER_BYTE_TRANSMITTING));

	I2C_GenerateSTOP(CODEC_I2C, ENABLE);
  WAIT_LONG(I2C_GetFlagStatus(CODEC_I2C, I2C_FLAG_BUSY));

	I2C_AcknowledgeConfig(CODEC_I2C, DISABLE);

	I2C_GenerateSTART(CODEC_I2C, ENABLE);
  WAIT(!I2C_CheckEvent(CODEC_I2C, I2C_EVENT_MASTER_MODE_SELECT));

	I2C_Send7bitAddress(CODEC_I2C, CODEC_ADDRESS, I2C_Direction_Receiver);
  WAIT(!I2C_CheckEvent(CODEC_I2C, I2C_EVENT_MASTER_RECEIVER_MODE_SELECTED));

  WAIT_LONG(!I2C_CheckEvent(CODEC_I2C, I2C_EVENT_MASTER_BYTE_RECEIVED));

	receivedByte = I2C_ReceiveData(CODEC_I2C);

	I2C_GenerateSTOP(CODEC_I2C, ENABLE);

	return receivedByte;
}

bool Codec::InitializeCodec(
    uint32_t sample_rate,
    CodecProtocol protocol,
    CodecFormat format) {
  bool s = true;  // success;
#ifdef CODEC_RESET_PIN
  GPIO_SetBits(CODEC_RESET_GPIO, CODEC_RESET_PIN);
#endif

#ifdef DAC_WM8731
  s = s && WriteControlRegister16(CODEC_REG_RESET, 0);
  // Configure L&R inputs
  s = s && WriteControlRegister16(CODEC_REG_LEFT_LINE_IN, CODEC_INPUT_0_DB);
  s = s && WriteControlRegister16(CODEC_REG_RIGHT_LINE_IN, CODEC_INPUT_0_DB);
  
  // Configure L&R headphone outputs
  s = s && WriteControlRegister16(CODEC_REG_LEFT_HEADPHONES_OUT, CODEC_HEADPHONES_MUTE);
  s = s && WriteControlRegister16(CODEC_REG_RIGHT_HEADPHONES_OUT, CODEC_HEADPHONES_MUTE);

  // Configure analog routing
  s = s && WriteControlRegister16(
      CODEC_REG_ANALOGUE_ROUTING,
      CODEC_MIC_MUTE | CODEC_ADC_LINE | CODEC_OUTPUT_DAC_ENABLE);

  // Configure digital routing
  s = s && WriteControlRegister16(CODEC_REG_DIGITAL_ROUTING, CODEC_DEEMPHASIS_NONE);

  // Configure power management
  s = s && WriteControlWriteControlRegister16Register(
      CODEC_REG_POWER_MANAGEMENT,
      CODEC_POWER_DOWN_OSCILLATOR | \
        CODEC_POWER_DOWN_CLOCK_OUTPUT | \
        CODEC_POWER_DOWN_MIC);
  
  uint8_t format_byte = CODEC_FORMAT_SLAVE;
  if (protocol == CODEC_PROTOCOL_PHILIPS) {
    format_byte |= CODEC_PROTOCOL_MASK_PHILIPS;
  } else if (protocol == CODEC_PROTOCOL_MSB_FIRST) {
    format_byte |= CODEC_PROTOCOL_MASK_MSB_FIRST;
  } else if (protocol == CODEC_PROTOCOL_LSB_FIRST) {
    format_byte |= CODEC_PROTOCOL_MASK_LSB_FIRST;
  }
  
  if (format == CODEC_FORMAT_16_BIT) {
    format_byte |= CODEC_FORMAT_MASK_16_BIT;
  } else if (format == CODEC_FORMAT_24_BIT) {
    format_byte |= CODEC_FORMAT_MASK_24_BIT;
  } else if (format == CODEC_FORMAT_32_BIT) {
    format_byte |= CODEC_FORMAT_MASK_32_BIT;
  }
  s = s && WriteControlRegister16(CODEC_REG_DIGITAL_FORMAT, format_byte);
  
  uint8_t rate_byte = 0;
  // According to the WM8731 datasheet, the 32kHz and 96kHz modes require the
  // master clock to be at 12.288 MHz (384 fs / 128 fs). The STM32F4 I2S clock
  // is always at 256 fs. So the 32kHz and 96kHz modes are achieved by
  // pretending that we are doing 48kHz, but with a slower or faster master
  // clock.
  rate_byte = sample_rate == 44100 ? CODEC_RATE_44K_44K : CODEC_RATE_48K_48K;
  s = s && WriteControlRegister16(CODEC_REG_SAMPLE_RATE, rate_byte);

  // For now codec is not active.
  s = s && WriteControlRegister16(CODEC_REG_ACTIVE, 0x00);
#endif
#ifdef DAC_CS43L22

	uint32_t delaycount;
	uint8_t regValue = 0xFF;

	GPIO_SetBits(GPIOD, CODEC_RESET_PIN);
	delaycount = 1000000;
	while (delaycount > 0)
	{
		delaycount--;
	}
	//keep codec OFF
	WriteControlRegister(CODEC_MAP_PLAYBACK_CTRL1, 0x01);

	//begin initialization sequence (p. 32)
	WriteControlRegister(0x00, 0x99);
	WriteControlRegister(0x47, 0x80);
	regValue = ReadControlRegister(0x32);
	WriteControlRegister(0x32, regValue | 0x80);
	regValue = ReadControlRegister(0x32);
	WriteControlRegister(0x32, regValue & (~0x80));
	WriteControlRegister(0x00, 0x00);
	//end of initialization sequence

	WriteControlRegister(CODEC_MAP_PWR_CTRL2, 0xAF);
	WriteControlRegister(CODEC_MAP_PLAYBACK_CTRL1, 0x70);
	WriteControlRegister(CODEC_MAP_CLK_CTRL, 0x81); //auto detect clock
	WriteControlRegister(CODEC_MAP_IF_CTRL1, 0x07);
	WriteControlRegister(0x0A, 0x00);
	WriteControlRegister(0x27, 0x00);
	WriteControlRegister(0x1A | CODEC_MAPBYTE_INC, 0x0A, 0x0A);
	WriteControlRegister(0x1F, 0x0F);
	WriteControlRegister(CODEC_MAP_PWR_CTRL1, 0x9E);

#endif
  
  return s;
}

bool Codec::InitializeDMA() {
  I2S_Cmd(CODEC_I2S, ENABLE);


  RCC_AHB1PeriphClockCmd(AUDIO_I2S_DMA_CLOCK, ENABLE);

  // DMA setup for TX.
  DMA_Cmd(AUDIO_I2S_DMA_STREAM, DISABLE);
  DMA_DeInit(AUDIO_I2S_DMA_STREAM);

  dma_init_tx_.DMA_Channel = AUDIO_I2S_DMA_CHANNEL;
  dma_init_tx_.DMA_PeripheralBaseAddr = AUDIO_I2S_DMA_DREG;
  dma_init_tx_.DMA_Memory0BaseAddr = (uint32_t)0;
  dma_init_tx_.DMA_DIR = DMA_DIR_MemoryToPeripheral;
  dma_init_tx_.DMA_BufferSize = (uint32_t)0xFFFE;
  dma_init_tx_.DMA_PeripheralInc = DMA_PeripheralInc_Disable;
  dma_init_tx_.DMA_MemoryInc = DMA_MemoryInc_Enable;
  dma_init_tx_.DMA_PeripheralDataSize = DMA_PeripheralDataSize_HalfWord;
  dma_init_tx_.DMA_MemoryDataSize = DMA_MemoryDataSize_HalfWord;
  dma_init_tx_.DMA_Mode = DMA_Mode_Circular;
  dma_init_tx_.DMA_Priority = DMA_Priority_High;
  dma_init_tx_.DMA_FIFOMode = DMA_FIFOMode_Disable;
  dma_init_tx_.DMA_FIFOThreshold = DMA_FIFOThreshold_1QuarterFull;
  dma_init_tx_.DMA_MemoryBurst = DMA_MemoryBurst_Single;
  dma_init_tx_.DMA_PeripheralBurst = DMA_PeripheralBurst_Single;
  DMA_Init(AUDIO_I2S_DMA_STREAM, &dma_init_tx_);

  // Enable the interrupts.
  DMA_ITConfig(AUDIO_I2S_DMA_STREAM, DMA_IT_TC | DMA_IT_HT, ENABLE);
    
  // Enable the IRQ.
  NVIC_EnableIRQ(AUDIO_I2S_DMA_IRQ);

#ifdef AUDIO_I2S_EXT_DMA_STREAM
  // DMA setup for RX.
  DMA_Cmd(AUDIO_I2S_EXT_DMA_STREAM, DISABLE);
  DMA_DeInit(AUDIO_I2S_EXT_DMA_STREAM);

  dma_init_rx_.DMA_Channel = AUDIO_I2S_EXT_DMA_CHANNEL;  
  dma_init_rx_.DMA_PeripheralBaseAddr = AUDIO_I2S_EXT_DMA_DREG;
  dma_init_rx_.DMA_Memory0BaseAddr = (uint32_t)0;
  dma_init_rx_.DMA_DIR = DMA_DIR_PeripheralToMemory;
  dma_init_rx_.DMA_BufferSize = (uint32_t)0xFFFE;
  dma_init_rx_.DMA_PeripheralInc = DMA_PeripheralInc_Disable;
  dma_init_rx_.DMA_MemoryInc = DMA_MemoryInc_Enable;
  dma_init_rx_.DMA_PeripheralDataSize = DMA_PeripheralDataSize_HalfWord;
  dma_init_rx_.DMA_MemoryDataSize = DMA_MemoryDataSize_HalfWord; 
  dma_init_rx_.DMA_Mode = DMA_Mode_Circular;
  dma_init_rx_.DMA_Priority = DMA_Priority_High;
  dma_init_rx_.DMA_FIFOMode = DMA_FIFOMode_Disable;         
  dma_init_rx_.DMA_FIFOThreshold = DMA_FIFOThreshold_1QuarterFull;
  dma_init_rx_.DMA_MemoryBurst = DMA_MemoryBurst_Single;
  dma_init_rx_.DMA_PeripheralBurst = DMA_PeripheralBurst_Single;  
  DMA_Init(AUDIO_I2S_EXT_DMA_STREAM, &dma_init_rx_);  

  // Enable the interrupts.
  DMA_ITConfig(AUDIO_I2S_EXT_DMA_STREAM, DMA_IT_TC | DMA_IT_HT, ENABLE);
    
  // Enable the IRQ.
  NVIC_EnableIRQ(AUDIO_I2S_EXT_DMA_IRQ);
#endif

  // Start DMA from/to codec.
  SPI_I2S_DMACmd(CODEC_I2S, SPI_I2S_DMAReq_Tx, ENABLE);
#ifdef CODEC_I2S_EXT
  SPI_I2S_DMACmd(CODEC_I2S_EXT, SPI_I2S_DMAReq_Rx, ENABLE);
#endif
  return true;
}

bool Codec::Init(
    uint32_t sample_rate,
    CodecProtocol protocol,
    CodecFormat format) {
  rx_buffer_.Init();
  tx_buffer_.Init();

  Frame s;
  s.l = s.r = 0;

  for (size_t i = 0; i < rx_buffer_.capacity() >> 1; ++i) {
    rx_buffer_.Overwrite(s);
    tx_buffer_.Overwrite(s);
  }

  instance_ = this;
  callback_ = NULL;

  return InitializeGPIO() && \
    InitializeControlInterface() && \
    InitializeAudioInterface(sample_rate, protocol, format) && \
    InitializeCodec(sample_rate, protocol, format) && \
    InitializeDMA();
}

bool Codec::Start(FillBufferCallback callback) {
  // Start the codec.
#ifdef DAC_WM8731
  if (!WriteControlRegister(CODEC_REG_ACTIVE, 0x01)) {
    return false;
  }
#endif

  callback_ = callback;
  client_tx_ = NULL;
  client_rx_ = NULL;
  transmitted_ = 0;
  processed_ = 0;
  
//   // Enable the I2S TX and RX peripherals.
//   if ((CODEC_I2S->I2SCFGR & 0x0400) == 0){
//     I2S_Cmd(CODEC_I2S, ENABLE);
//   }
// #ifdef CODEC_I2S_EXT
//   if ((CODEC_I2S_EXT->I2SCFGR & 0x0400) == 0){
//     I2S_Cmd(CODEC_I2S_EXT, ENABLE);
//   }
// #endif

  dma_init_tx_.DMA_Memory0BaseAddr = (uint32_t)(tx_dma_buffer_);
  dma_init_rx_.DMA_Memory0BaseAddr = (uint32_t)(rx_dma_buffer_);

  dma_init_tx_.DMA_BufferSize = kAudioChunkSize * 2 * 2;
  dma_init_rx_.DMA_BufferSize = kAudioChunkSize * 2 * 2;
  
  DMA_Init(AUDIO_I2S_DMA_STREAM, &dma_init_tx_);
#ifdef AUDIO_I2S_EXT_DMA_STREAM
  DMA_Init(AUDIO_I2S_EXT_DMA_STREAM, &dma_init_rx_);
#endif
  DMA_Cmd(AUDIO_I2S_DMA_STREAM, ENABLE);
#ifdef AUDIO_I2S_EXT_DMA_STREAM
  DMA_Cmd(AUDIO_I2S_EXT_DMA_STREAM, ENABLE);
#endif
  
  return true;
}

void Codec::Stop() {
  DMA_Cmd(AUDIO_I2S_DMA_STREAM, DISABLE);
#ifdef AUDIO_I2S_EXT_DMA_STREAM
  DMA_Cmd(AUDIO_I2S_EXT_DMA_STREAM, DISABLE);
#endif
}

void Codec::Fill(size_t offset) {
  if (kNumFIFOChunks) {
    // Write input samples to FIFO, Read output samples from FIFO
    rx_buffer_.Overwrite(&rx_dma_buffer_[offset], kAudioChunkSize);
    tx_buffer_.ImmediateRead(&tx_dma_buffer_[offset], kAudioChunkSize);
  } else if (callback_) {
    (*callback_)(
        &rx_dma_buffer_[offset],
        &tx_dma_buffer_[offset],
        kAudioChunkSize);
  } else {
    // Inform the client that some data is ready, and store pointers to the
    // valid section of the ring buffer.
    ++transmitted_;
    client_rx_ = &rx_dma_buffer_[offset];
    client_tx_ = &tx_dma_buffer_[offset];
  }
}

}  // namespace elements

extern "C" {
// Do not call into the firmware library to save on calls/jumps.
// if (DMA_GetFlagStatus(AUDIO_I2S_EXT_DMA_STREAM, AUDIO_I2S_EXT_DMA_FLAG_TC) != RESET) {
//  DMA_ClearFlag(AUDIO_I2S_EXT_DMA_STREAM, AUDIO_I2S_EXT_DMA_FLAG_TC);  

#ifdef AUDIO_I2S_EXT_DMA_REG
void DMA1_Stream3_IRQHandler(void) {
  if (AUDIO_I2S_EXT_DMA_REG->AUDIO_I2S_EXT_DMA_ISR & AUDIO_I2S_EXT_DMA_FLAG_TC) {
    AUDIO_I2S_EXT_DMA_REG->AUDIO_I2S_EXT_DMA_IFCR = AUDIO_I2S_EXT_DMA_FLAG_TC;
    elements::Codec::GetInstance()->Fill(elements::kAudioChunkSize);
  }
  if (AUDIO_I2S_EXT_DMA_REG->AUDIO_I2S_EXT_DMA_ISR & AUDIO_I2S_EXT_DMA_FLAG_HT) {
    AUDIO_I2S_EXT_DMA_REG->AUDIO_I2S_EXT_DMA_IFCR = AUDIO_I2S_EXT_DMA_FLAG_HT;
    elements::Codec::GetInstance()->Fill(0);
  }
}
#else
void DMA1_Stream5_IRQHandler(void) {
  if (AUDIO_I2S_DMA_REG->AUDIO_I2S_DMA_ISR & AUDIO_I2S_DMA_FLAG_TC) {
    AUDIO_I2S_DMA_REG->AUDIO_I2S_DMA_IFCR = AUDIO_I2S_DMA_FLAG_TC;
    elements::Codec::GetInstance()->Fill(elements::kAudioChunkSize);

  }
  if (AUDIO_I2S_DMA_REG->AUDIO_I2S_DMA_ISR & AUDIO_I2S_DMA_FLAG_HT) {
    AUDIO_I2S_DMA_REG->AUDIO_I2S_DMA_IFCR = AUDIO_I2S_DMA_FLAG_HT;
    elements::Codec::GetInstance()->Fill(0);
  }
}
#endif
  
}
