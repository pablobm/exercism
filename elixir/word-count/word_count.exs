defmodule Words do
  @doc """
  Count the number of words in the sentence.

  Words are compared case-insensitively.
  """
  @spec count(String.t) :: map
  def count(sentence) do
    sentence
    |> String.downcase
    |> String.replace(~r/[^-\p{L}\p{N} ]+/u, " ")
    |> String.split(" ")
    |> Enum.filter(fn(word) -> word != "" end)
    |> count_words
  end

  @spec count_words([String.t]) :: map
  defp count_words([word | t]) do
    Map.update(count_words(t), word, 1, &(&1 + 1))
  end

  @spec count_words([]) :: map
  defp count_words([]) do
    %{}
  end
end
