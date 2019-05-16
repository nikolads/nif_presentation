defmodule MinimalExampleTest do
  use ExUnit.Case
  doctest MinimalExample

  test "greets the world" do
    assert MinimalExample.hello() == :world
  end
end
