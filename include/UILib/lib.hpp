#include <string>

/// Options to pass when starting an app.
class AppStartupOptions {
public:
  bool useVsync = true;
  int antiAliasingLevel = 4;
  AppStartupOptions();
  AppStartupOptions(bool useVsync) : useVsync(useVsync) {};
  AppStartupOptions(bool useVsync, int antiAliasingLevel)
      : useVsync(useVsync), antiAliasingLevel(antiAliasingLevel) {};
};

void runApp(std::string appName);
void runApp(std::string appName, AppStartupOptions options);
