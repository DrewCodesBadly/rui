#include <SFML/Graphics.hpp>
#include <SFML/Graphics/Color.hpp>
#include <SFML/Graphics/RenderWindow.hpp>
#include <SFML/System/String.hpp>
#include <SFML/Window/ContextSettings.hpp>
#include <SFML/Window/Event.hpp>
#include <SFML/Window/WindowEnums.hpp>
#include <UILib/lib.hpp>
#include <string>

void runApp(std::string appName) { runApp(appName, AppStartupOptions()); }

void runApp(std::string appName, AppStartupOptions options) {
  sf::ContextSettings settings;
  settings.antiAliasingLevel = 4; // seems to be the only relevant setting
  sf::RenderWindow window(sf::VideoMode({800, 600}), appName,
                          sf::Style::Default, sf::State::Windowed, settings);
  window.setVerticalSyncEnabled(true);
  while (window.isOpen()) {
    while (const std::optional event = window.pollEvent()) {
      // Example: listening to different types of events and "handling" them
      if (event->is<sf::Event::Closed>()) {
        window.close();
      } else if (const auto *resized = event->getIf<sf::Event::Resized>()) {
      } else if (const auto *keyPressed =
                     event->getIf<sf::Event::KeyPressed>()) {
      } else if (const auto *keyRelease =
                     event->getIf<sf::Event::KeyReleased>()) {
      } else if (const auto *mousePressed =
                     event->getIf<sf::Event::MouseButtonPressed>()) {
      } else if (const auto *mouseReleased =
                     event->getIf<sf::Event::MouseButtonReleased>()) {
      } else if (const auto *mouseMoved =
                     event->getIf<sf::Event::MouseMoved>()) {
      } else if (const auto *mouseScrolled =
                     event->getIf<sf::Event::MouseWheelScrolled>()) {
      } else if (const auto *textEntered =
                     event->getIf<sf::Event::TextEntered>()) {
      } else if (const auto *resized = event->getIf<sf::Event::Resized>()) {
      }
    }

    window.clear(sf::Color::White);

    // this is where the redraw loop goes if we're clearing and redrawing per
    // frame.

    window.display();
  }
}
