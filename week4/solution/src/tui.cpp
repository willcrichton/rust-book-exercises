#include <iostream>
#include <memory>
#include <string>
#include <vector>

using namespace std;

struct Dimensions {
  size_t width;
  size_t height;
};

class Element {
public:
  virtual Dimensions dimensions() = 0;
  virtual void render() = 0;
  virtual ~Element() = default;
};

class Text : public Element {
protected:
  string text;

public:
  Text(string text) : text(text) {}

  Dimensions dimensions() { return {text.size(), 1}; }

  void render() { cout << text; }
};

class Heading : public Text {
public:
  using Text::Text;

  void render() { cout << "\u001b[1m" << text << "\u001b[0m"; }
};

class Container : public Element {
  vector<unique_ptr<Element>> children;

public:
  Container(vector<unique_ptr<Element>> &children) : children(move(children)) {}

  Dimensions dimensions() {
    size_t max_width = 0;
    size_t sum_height = 0;
    for (auto &child : children) {
      auto dims = child->dimensions();
      max_width = max(max_width, dims.width);
      sum_height += dims.height;
    }
    return {max_width + 2, sum_height};
  }

  void render() {
    auto dims = dimensions();
    auto render_line = [&] {
      cout << "+";
      for (size_t i = 0; i < dims.width - 2; ++i)
        cout << "-";
      cout << "+" << endl;
    };
    render_line();

    for (auto &child : children) {
      auto child_dims = child->dimensions();
      cout << "|";
      child->render();
      for (size_t i = 0; i < dims.width - 2 - child_dims.width; ++i)
        cout << " ";
      cout << "|" << endl;
    }

    render_line();
  }
};

int main() {
  unique_ptr<Element> text = make_unique<Heading>("Hello world");
  unique_ptr<Element> text2 =
      make_unique<Text>("This is a long string of text");

  vector<unique_ptr<Element>> children;
  children.push_back(move(text));
  children.push_back(move(text2));
  unique_ptr<Element> container = make_unique<Container>(children);

  container->render();

  return 0;
}