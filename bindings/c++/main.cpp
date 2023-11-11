#include <iostream>
#include <LandscapeToolkit/LandscapeToolkit>

int main()
{
  LandscapeToolkit::Internal::enableLogger();
  std::cout << std::boolalpha
            << LandscapeToolkit::loadDirectory("elevations", LandscapeToolkit::LoadMode::Relative) << std::endl;
  auto a = LandscapeToolkit::elevationAt(60.1, 30.1);
  if(a.has_value())
    std::cout << a.value() << std::endl;
  else
    std::cerr << a.error().message() << std::endl;
  return 0;
}
