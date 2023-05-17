defmodule HelloRust do
  use Rustler, otp_app: :hello_rust, crate: :hello

  def hello, do: :erlang.nif_error(:nif_not_loaded)
end
