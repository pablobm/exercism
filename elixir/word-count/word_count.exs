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
    |> Enum.filter(&(&1 != ""))
    |> count_words
  end

  @spec count_words([String.t]) :: map
  defp count_words([word | t]) do
    count_words(t)
    |> Map.update(word, 1, &(&1 + 1))
  end

  @spec count_words([]) :: map
  defp count_words([]) do
    %{}
  end
end
