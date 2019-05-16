defmodule Add do
  use Rustler, otp_app: :add, crate: "add"

  def add(_a, _b), do: :erlang.nif_error(:nif_not_loaded)
  def add_v2(_a, _b), do: :erlang.nif_error(:nif_not_loaded)
  def add_v3(_a, _b), do: :erlang.nif_error(:nif_not_loaded)
end
