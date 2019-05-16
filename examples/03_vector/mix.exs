defmodule Vector.MixProject do
  use Mix.Project

  def project do
    [
      app: :vector,
      version: "0.1.0",
      elixir: "~> 1.8",
      start_permanent: Mix.env() == :prod,
      compilers: [:rustler | Mix.compilers()],
      deps: deps(),
      rustler_crates: rustler_crates(),
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:rustler, "~> 0.20.0" },
    ]
  end

  defp rustler_crates do
    [
      vector: [],
    ]
  end
end
