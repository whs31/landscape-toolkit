#include <gtest/gtest.h>
#include <LandscapeToolkit/LandscapeToolkit>
#include <Libra/Platform>

#if defined(LIBRA_OS_WINDOWS)
constexpr const char* TEST_DATA_DIRECTORY = R"(..\..\..\testdata\elevations)";
#else
constexpr const char* TEST_DATA_DIRECTORY = "../../../testdata/elevations";
#endif

TEST(Basic, Directory)
{
  EXPECT_TRUE(LandscapeToolkit::Private::enableLogger());
  EXPECT_TRUE(LandscapeToolkit::loadDirectory(TEST_DATA_DIRECTORY, LandscapeToolkit::DirectoryPathMode::Relative));
}

TEST(Basic, Version)
{
  EXPECT_EQ(LandscapeToolkit::Private::version(), "0.1.0");
}

TEST(Basic, BinaryDirectory)
{
  EXPECT_STREQ(LandscapeToolkit::binaryDirectory().c_str(), "./");
}

TEST(Basic, ElevationAt)
{

  EXPECT_TRUE(LandscapeToolkit::setDirectory(TEST_DATA_DIRECTORY, LandscapeToolkit::DirectoryPathMode::Relative));
  EXPECT_FLOAT_EQ(LandscapeToolkit::elevationAt(60.0, 30.0, LandscapeToolkit::PreloadMode::PreloadTile).value_or(-1.0f), 0.0f);
  EXPECT_FLOAT_EQ(LandscapeToolkit::elevationAt(60.9, 30.9, LandscapeToolkit::PreloadMode::PreloadTile).value_or(-1.0f), 3.0f);
  EXPECT_FLOAT_EQ(LandscapeToolkit::elevationAt(60.5, 30.5, LandscapeToolkit::PreloadMode::PreloadTile).value_or(-1.0f), 66.0f);
  EXPECT_TRUE(LandscapeToolkit::elevationAt(6.5, 30.5, LandscapeToolkit::PreloadMode::PreloadTile).has_value() == false);
}