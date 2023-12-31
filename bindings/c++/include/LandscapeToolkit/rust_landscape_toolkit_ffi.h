#pragma once

#include <Libra/Platform>

#if defined(LIBRA_OS_WINDOWS)
  #define RUST_LED_TOOLKIT_IMPORT __declspec(dllimport)
#elif defined(LIBRA_OS_LINUX)
  #define RUST_LED_TOOLKIT_IMPORT
#else
  #error "Unknown architecture.";
#endif

extern "C" struct RUST_LED_TOOLKIT_IMPORT LEDResult
{
  float result;
  bool valid;
};

extern "C" struct RUST_LED_TOOLKIT_IMPORT LEDVersion
{
  int major;
  int minor;
  int patch;
};

extern "C" RUST_LED_TOOLKIT_IMPORT LEDVersion led_version();
extern "C" RUST_LED_TOOLKIT_IMPORT const char* led_binary_directory();
extern "C" RUST_LED_TOOLKIT_IMPORT bool led_init_logger();
extern "C" RUST_LED_TOOLKIT_IMPORT bool led_load_relative_directory(const char*);
extern "C" RUST_LED_TOOLKIT_IMPORT bool led_load_directory(const char*);
extern "C" RUST_LED_TOOLKIT_IMPORT bool led_set_relative_directory(const char*);
extern "C" RUST_LED_TOOLKIT_IMPORT bool led_set_directory(const char*);
extern "C" RUST_LED_TOOLKIT_IMPORT LEDResult led_elevation_at(double, double, int);
extern "C" RUST_LED_TOOLKIT_IMPORT int led_elevation_at_as_int(double, double, int);
