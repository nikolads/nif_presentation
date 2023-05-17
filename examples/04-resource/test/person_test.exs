defmodule PersonTest do
  use ExUnit.Case
  doctest Person

  test "it works" do
    ivancho = Person.make("Иванчо", 123)

    assert Person.get_name(ivancho) == "Иванчо"
    assert Person.get_age(ivancho) == 123
  end
end
