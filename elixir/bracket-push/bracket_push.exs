defmodule BracketPush do
  @doc """
  Checks that all the brackets and braces in the string are matched correctly, and nested correctly
  """
  @spec check_brackets(String.t) :: boolean
  def check_brackets(str) do
    check(String.graphemes(str), [])
  end

  def check([], []) do
    true
  end

  def check([], _) do
    false
  end

  def check([h | t1], [h | t2]) do
    check(t1, t2)
  end

  def check([h | t], openings) do
    cond do
      is_opener?(h)
        -> check(t, [closer_for(h) | openings])
      is_closer?(h)
        -> false
      true
        -> check(t, openings)
    end
  end

  defp is_opener?(char) do
    MapSet.member?(openers, char)
  end

  defp is_closer?(char) do
    MapSet.member?(closers, char)
  end

  defp closer_for(opener) do
    openers_to_closers[opener]
  end

  defp openers do
    MapSet.new(Map.keys(openers_to_closers))
  end

  defp closers do
    MapSet.new(Map.values(openers_to_closers))
  end

  defp openers_to_closers do
    %{
      "{" => "}",
      "[" => "]",
      "(" => ")",
    }
  end
end
