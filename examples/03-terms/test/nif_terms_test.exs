defmodule NifTermsTest do
  use ExUnit.Case
  doctest NifTerms

  test "inspects all the terms" do
    assert NifTerms.inspect(:atom) ==
             {:ok, ~s(got :atom)}

    assert NifTerms.inspect("binary") ==
             {:ok, ~s(got "binary")}

    assert NifTerms.inspect([]) ==
             {:ok, ~s(got [])}

    assert NifTerms.inspect([:atom, "binary"]) ==
             {:ok, ~s(got [:atom, "binary"])}

    assert NifTerms.inspect([:atom, [:inner, [], "list"], "binary"]) ==
             {:ok, ~s(got [:atom, [:inner, [], "list"], "binary"])}

    assert NifTerms.inspect([:atom, "string", 123]) ==
             {:error, :unknown_term}

    assert NifTerms.inspect(123) ==
             {:error, :unknown_term}
  end

  test "inspect_v2s all the terms" do
    assert NifTerms.inspect_v2(:atom) ==
             {:ok, ~s(got :atom)}

    assert NifTerms.inspect_v2("binary") ==
             {:ok, ~s(got "binary")}

    assert NifTerms.inspect_v2([]) ==
             {:ok, ~s(got [])}

    assert NifTerms.inspect_v2([:atom, "binary"]) ==
             {:ok, ~s(got [:atom, "binary"])}

    assert NifTerms.inspect_v2([:atom, [:inner, [], "list"], "binary"]) ==
             {:ok, ~s(got [:atom, [:inner, [], "list"], "binary"])}

    assert NifTerms.inspect_v2([:atom, "string", 123]) ==
             {:error, :unknown_term}

    assert NifTerms.inspect_v2(123) ==
             {:error, :unknown_term}
  end

  test "foobar" do
    assert NifTerms.get_foobar(:foo) == :foo
    assert NifTerms.get_foobar(:bar) == {:bar, "bar"}
    assert NifTerms.get_foobar(:baz) == {:baz, %{a: 12, b: 23}}
  end

  test "foobar invalid" do
    catch_error(NifTerms.get_foobar(123))
    catch_error(NifTerms.get_foobar(:invalid))
  end
end
