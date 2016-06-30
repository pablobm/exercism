defmodule RunLengthEncoder do
  @doc """
  Generates a string where consecutive elements are represented as a data value and count.
  "HORSE" => "1H1O1R1S1E"
  For this example, assume all input are strings, that are all uppercase letters.
  It should also be able to reconstruct the data into its original form.
  "1H1O1R1S1E" => "HORSE"
  """
  @spec encode(String.t) :: String.t
  def encode(string) do
    string
    |> String.graphemes
    |> split_into_identical_groups
    |> to_count_and_item_pairs
    |> List.flatten
    |> Enum.join
  end

  @spec decode(String.t) :: String.t
  def decode(string) do
    Regex.scan(~r/\p{N}+|\P{N}+/, string)
    |> List.flatten
    |> numbers_as_numbers
    |> to_pairs
    |> expand_to_string
  end

  defp split_into_identical_groups(items) do
    Enum.chunk_by(items, &(&1))
  end

  defp to_count_and_item_pairs(items) do
    Enum.map(items, &(
      [Integer.to_string(length(&1)), List.first(&1)]
    ))
  end

  defp numbers_as_numbers(graphemes) do
    Enum.map graphemes, fn (g) ->
      if Regex.match?(~r/\p{N}/, g) do
        String.to_integer(g)
      else
        g
      end
    end
  end

  defp to_pairs([first, second | rest]) do
    Enum.concat([[first, second]], to_pairs(rest))
  end

  defp to_pairs([]) do
    []
  end

  defp expand_to_string([[count, grapheme] | rest]) do
    String.duplicate(grapheme, count) <> expand_to_string(rest)
  end

  defp expand_to_string([]) do
    ""
  end

end
