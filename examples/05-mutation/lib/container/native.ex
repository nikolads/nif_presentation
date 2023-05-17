defmodule Container.Native do
  use Rustler, otp_app: :mutation, crate: "container_native"

  def new(), do: :erlang.nif_error(:nif_not_loaded)
  def insert(_container, _key, _val), do: :erlang.nif_error(:nif_not_loaded)
  def get(_container, _key), do: :erlang.nif_error(:nif_not_loaded)
end
