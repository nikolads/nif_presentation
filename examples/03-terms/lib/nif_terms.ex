defmodule NifTerms do
  use Rustler, otp_app: :terms

  def inspect(_term), do: :erlang.nif_error(:nif_not_loaded)
  def inspect_v2(_term), do: :erlang.nif_error(:nif_not_loaded)
  def get_foobar(_kind), do: :erlang.nif_error(:nif_not_loaded)
end
