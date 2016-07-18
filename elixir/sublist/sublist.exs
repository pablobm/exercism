defmodule Sublist do
  @doc """
  Returns whether the first list is a sublist or a superlist of the second list
  and if not whether it is equal or unequal to the second list.
  """
  def compare(list, list) do
    :equal
  end

  def compare(l1, l2) do
    cond do
      length(l1) < length(l2) && l1 |> is_sublist_of(l2)
        -> :sublist
      length(l1) > length(l2) && l2 |> is_sublist_of(l1)
        -> :superlist
      true
        -> :unequal
    end
  end

  defp is_sublist_of([], list) do
    true
  end

  defp is_sublist_of(list, []) do
    false
  end

  defp is_sublist_of(l1, l2 = [h2 | t2]) do
    l2 |> starts_with(l1) || l1 |> is_sublist_of(t2)
  end

  defp starts_with([h | t1], [h | t2]) do
    t1 |> starts_with(t2)
  end

  defp starts_with(_, []) do
    true
  end

  defp starts_with(_, _) do
    false
  end
end
