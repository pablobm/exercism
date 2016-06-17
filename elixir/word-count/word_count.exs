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
    counts = count_words(t)
    word_count = counts[word] || 0
    Map.put(counts, word, word_count + 1)
  end

  defp count_words([]) do
    %{}
  end
end