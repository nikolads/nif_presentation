defmodule HelloCTest do
  use ExUnit.Case
  doctest HelloC

  test "greets the world" do
    assert HelloC.hello() == "Здравей от C!"
  end
end
