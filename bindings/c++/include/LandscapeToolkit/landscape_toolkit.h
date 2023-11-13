#pragma once

#include <string>
#include <system_error>
#include <Libra/Global>
#include <Libra/Expected>
#include "rust_landscape_toolkit_ffi.h"

using std::string;

namespace LandscapeToolkit
{
  enum class DirectoryPathMode
  {
      Absolute,
      Relative
  };

  enum class PreloadMode
  {
      PreloadTile = 1,
      NoPreload = 2
  };

  namespace Private
  {
    auto enableLogger() -> bool { return led_init_logger(); }
    auto version() -> string
    {
      auto vs = led_version();
      return std::to_string(vs.major) + "." + std::to_string(vs.minor) + "." + std::to_string(vs.patch);
    }
  } // Internal

  auto loadDirectory(const string& path, DirectoryPathMode mode) -> bool
  {
    switch(mode)
    {
      case DirectoryPathMode::Absolute: return led_load_directory(path.c_str());
      case DirectoryPathMode::Relative: return led_load_relative_directory(path.c_str());
    }
  }

  auto setDirectory(const string& path, DirectoryPathMode mode) -> bool
  {
    switch(mode)
    {
      case DirectoryPathMode::Absolute: return led_set_directory(path.c_str());
      case DirectoryPathMode::Relative: return led_set_relative_directory(path.c_str());
    }
  }

  auto elevationAt(f64 latitude, f64 longitude, PreloadMode mode) -> expected<f32, std::error_code>
  {
    auto a = led_elevation_at(latitude, longitude, static_cast<int>(mode));
    //if(a.valid)
      return a.result;
    return unexpected(std::make_error_code(std::errc::bad_message));
  }
} // LandscapeToolkit