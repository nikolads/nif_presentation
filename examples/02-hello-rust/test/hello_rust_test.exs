defmodule HelloRustTest do
  use ExUnit.Case
  doctest HelloRust

  test "greets the world" do
    assert HelloRust.hello() == "Здравей от Rust!"
  end
end
