defmodule VectorNif do
  use Rustler, otp_app: :vector, crate: "vector"

  def new, do: :erlang.nif_error(:nif_not_loaded)
  def len(_vec), do: :erlang.nif_error(:nif_not_loaded)
  def push(_vec, _item), do: :erlang.nif_error(:nif_not_loaded)
  def get(_vec, _index), do: :erlang.nif_error(:nif_not_loaded)
end
