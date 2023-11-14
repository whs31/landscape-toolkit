using System;
using System.IO;
using System.Runtime.InteropServices;

namespace HSF.Interop
{
  public static class LandscapeInterface
  {
      internal static class NativeLibraryInterface
      {
        [StructLayout(LayoutKind.Sequential, CharSet = CharSet.Ansi)] 
        public struct LEDVersion
        {
          public int major;
          public int minor;
          public int patch;
        }
        
        [DllImport("landscape_toolkit", CallingConvention = CallingConvention.Cdecl)]
        public static extern LEDVersion led_version();
        
        [DllImport("landscape_toolkit", CallingConvention = CallingConvention.Cdecl)]
        public static extern IntPtr led_binary_directory();
        
        [DllImport("landscape_toolkit", CallingConvention = CallingConvention.Cdecl)]
        public static extern bool led_init_logger();
        
        [DllImport("landscape_toolkit", CallingConvention = CallingConvention.Cdecl)]
        public static extern bool led_load_relative_directory(IntPtr str);
        
        [DllImport("landscape_toolkit", CallingConvention = CallingConvention.Cdecl)]
        public static extern bool led_load_directory(IntPtr str);
        
        [DllImport("landscape_toolkit", CallingConvention = CallingConvention.Cdecl)]
        public static extern bool led_set_relative_directory(IntPtr str);
        
        [DllImport("landscape_toolkit", CallingConvention = CallingConvention.Cdecl)]
        public static extern bool led_set_directory(IntPtr str);
        
        [DllImport("landscape_toolkit", CallingConvention = CallingConvention.Cdecl)]
        public static extern int led_elevation_at_as_int(double lat, double lon, int pm);
      }
      
      public enum DirectoryPathMode
      {
        Absolute, 
        Relative
      }
      
      public enum PreloadMode
      {
        PreloadTile = 1,
        NoPreload = 2
      }
      
      public static string Version()
      {
        var v = NativeLibraryInterface.led_version();
        return $"{v.major}.{v.minor}.{v.patch}";
      } 
      
      public static string BinaryDirectory()
      {
        return Marshal.PtrToStringAnsi(NativeLibraryInterface.led_binary_directory());
      }
      
      public static bool LoadDirectory(string path, DirectoryPathMode mode)
      {
        return mode switch
        {
          DirectoryPathMode.Absolute => NativeLibraryInterface.led_load_directory(Marshal.StringToHGlobalAnsi(path)), 
          DirectoryPathMode.Relative => NativeLibraryInterface.led_load_relative_directory(Marshal.StringToHGlobalAnsi(path)),
          _ => throw new ArgumentOutOfRangeException(nameof(mode), mode, "Invalid enum value.")
        };
      }
      
      public static bool SetDirectory(string path, DirectoryPathMode mode)
      {
        return mode switch
        {
          DirectoryPathMode.Absolute => NativeLibraryInterface.led_set_directory(Marshal.StringToHGlobalAnsi(path)),
          DirectoryPathMode.Relative => NativeLibraryInterface.led_set_relative_directory(Marshal.StringToHGlobalAnsi(path)),
          _ => throw new ArgumentOutOfRangeException(nameof(mode), mode, "Invalid enum value.")
        };
      }
      
      public static float ElevationAt(double latitude, double longitude, PreloadMode mode)
      {
        var x = NativeLibraryInterface.led_elevation_at_as_int(latitude, longitude, (int)mode);
        if(x != 404) 
          return x;
        throw new FileNotFoundException("Tile not found or library failed to get elevation value from it.");
      }
      
      internal static bool EnableLogger() { return NativeLibraryInterface.led_init_logger(); }
  }
}