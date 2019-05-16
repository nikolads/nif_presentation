defmodule Vector do
  use Agent

  def start_link() do
    Agent.start_link(fn -> VectorNif.new end)
  end

  def len(pid) do
    Agent.get(pid, fn vec -> VectorNif.len(vec) end)
  end

  def get(pid, index) do
    Agent.get(pid, fn vec -> VectorNif.get(vec, index) end)
  end

  def push(pid, item) do
    Agent.get(pid, fn vec -> VectorNif.push(vec, item) end)
  end
end
