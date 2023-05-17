defmodule Person do
  use Rustler, otp_app: :resource, crate: :resource

  def make(_name, _age), do: :erlang.nif_error(:nif_not_loaded)
  def get_name(_person), do: :erlang.nif_error(:nif_not_loaded)
  def get_age(_person), do: :erlang.nif_error(:nif_not_loaded)
end
