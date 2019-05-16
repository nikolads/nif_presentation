defmodule Scheduling do
  use Rustler, otp_app: :scheduling, crate: "scheduling"

  def do_work, do: :erlang.nif_error(:nif_not_loaded)
  def echo_msg(_pid, _msg), do: :erlang.nif_error(:nif_not_loaded)
  def echo_from_thread(_msg), do: :erlang.nif_error(:nif_not_loaded)
  def dirty_nif, do: :erlang.nif_error(:nif_not_loaded)
end
