defmodule Container do
  use GenServer

  @type t :: %__MODULE__{inner: Reference.t()}
  @enforce_keys [:inner]
  defstruct [:inner]

  def start_link() do
    GenServer.start_link(__MODULE__, nil)
  end

  def init(_) do
    inner = Container.Native.new()
    {:ok, %__MODULE__{inner: inner}}
  end

  def handle_call({:insert, key, val}, _from, %__MODULE__{} = state)
      when is_binary(key) and is_binary(val) do
    result = state.inner |> Container.Native.insert(key, val)
    {:reply, result, state}
  end

  def handle_call({:get, key}, _from, %__MODULE__{} = state) when is_binary(key) do
    val = state.inner |> Container.Native.get(key)
    {:reply, val, state}
  end
end
