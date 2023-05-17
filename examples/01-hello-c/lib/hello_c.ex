defmodule HelloC do
  @on_load :load_nifs
  def load_nifs do
    :ok = :erlang.load_nif("./nif/_build/libhello", 0)
  end

  def hello, do: :erlang.nif_error(:nif_not_loaded)
end
