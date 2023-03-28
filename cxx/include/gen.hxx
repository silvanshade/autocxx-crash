#pragma once

#include <string>

class Test
{
public:
  explicit Test(std::string string)
    : string(std::move(string))
  {
  }

  Test() = delete;

  [[nodiscard]] auto get_string() const -> std::string const& { return this->string; }

private:
  std::string string;
};
