defmodule Anagram do
  @doc """
  Returns all candidates that are anagrams of, but not equal to, 'base'.
  """
  @spec match(String.t, [String.t]) :: [String.t]
  def match(base, candidates) do
    downcased_base = String.downcase(base)
    candidates
    |> Enum.reject(&(String.equivalent?(downcased_base, String.downcase(&1))))
    |> Enum.filter(&(is_anagram?(downcased_base, &1)))
  end

  defp is_anagram?(word, candidate) do
    ordered_letters(word) == ordered_letters(candidate)
  end

  defp ordered_letters(string) do
    string
    |> String.downcase
    |> String.graphemes
    |> Enum.sort
  end
end
