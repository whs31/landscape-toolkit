#include <gtest/gtest.h>
#include <LandscapeToolkit/LandscapeToolkit>

TEST(Basic, Directory)
{
  EXPECT_TRUE(LandscapeToolkit::loadDirectory("../../../testdata/elevations", LandscapeToolkit::LoadMode::Relative));
}