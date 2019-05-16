defmodule Niftest do
  def load_nifs do
    :erlang.load_nif("./niftest", 0)
  end

  def hello do
    :erlang.nif_error(:nif_not_loaded)
  end
end
