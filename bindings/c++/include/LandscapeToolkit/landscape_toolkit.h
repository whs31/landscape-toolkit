#pragma once

#include <string>
#include <system_error>
#include <Libra/Global>
#include <Libra/Expected>
#include "rust_landscape_toolkit_ffi.h"

using std::string;

namespace LandscapeToolkit
{
  enum class LoadMode
  {
      Absolute,
      Relative
  };

  namespace Internal
  {
    void enableLogger() { led_init_logger(); }
  } // Internal

  auto loadDirectory(const string& path, LoadMode mode) -> bool
  {
    switch(mode)
    {
      case LoadMode::Absolute: return led_load_directory(path.c_str());
      case LoadMode::Relative: return led_load_relative_directory(path.c_str());
    }
  }

  auto elevationAt(f64 latitude, f64 longitude) -> expected<f32, std::error_code>
  {
    auto a = led_elevation_at(latitude, longitude);
    if(a.valid)
      return a.result;
    return unexpected(std::make_error_code(std::errc::bad_message));
  }
} // LandscapeToolkit