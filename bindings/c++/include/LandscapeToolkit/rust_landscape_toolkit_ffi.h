#pragma once

#include <Libra/Platform>

#if defined(LIBRA_OS_WINDOWS)
  #define RUST_LED_TOOLKIT_IMPORT __declspec(dllimport)
#elif defined(LIBRA_OS_LINUX)
  #define RUST_LED_TOOLKIT_IMPORT
#else
  #error "Unknown architecture.";
#endif

extern "C" RUST_LED_TOOLKIT_IMPORT struct LEDResult
{
  float result;
  bool valid;
};

extern "C" RUST_LED_TOOLKIT_IMPORT void led_init_logger();
extern "C" RUST_LED_TOOLKIT_IMPORT bool led_load_relative_directory(const char*);
extern "C" RUST_LED_TOOLKIT_IMPORT bool led_load_directory(const char*);
extern "C" RUST_LED_TOOLKIT_IMPORT LEDResult led_elevation_at(double, double);
