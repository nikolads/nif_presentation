defmodule ContainerTest do
  use ExUnit.Case
  doctest Container.Native

  test "container works" do
    container = Container.Native.new()

    assert Container.Native.insert(container, "key1", "val1") == :inserted
    assert Container.Native.insert(container, "key2", "val2") == :inserted

    assert Container.Native.get(container, "key1") == "val1"
    assert Container.Native.get(container, "key3") == nil

    assert Container.Native.insert(container, "key1", "val11") == :occupied
    assert Container.Native.get(container, "key1") == "val1"
  end
end
