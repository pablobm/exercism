defmodule Acronym do
  @doc """
  Generate an acronym from a string.
  "This is a string" => "TIAS"
  """
  @spec abbreviate(string) :: String.t()
  def abbreviate(string) do
    Regex.scan(~r/[A-z][a-z]+/, string)
    |> List.flatten
    |> Enum.map(fn word -> String.at(word, 0) end)
    |> Enum.join
    |> String.upcase
  end
end
