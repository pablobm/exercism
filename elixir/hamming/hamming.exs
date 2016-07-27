defmodule DNA do
  @doc """
  Returns number of differences between two strands of DNA, known as the Hamming Distance.

  ## Examples

  iex> DNA.hamming_distance('AAGTCATA', 'TAGCGATC')
  {:ok, 4}
  """
  @spec hamming_distance([char], [char]) :: non_neg_integer
  def hamming_distance([], []) do
    {:ok, 0}
  end

  def hamming_distance(_, []) do
    error
  end

  def hamming_distance([], _) do
    error
  end

  def hamming_distance([h | t1], [h | t2]) do
    hamming_distance(t1, t2)
  end

  def hamming_distance([_ | t1], [_ | t2]) do
    case hamming_distance(t1, t2) do
      {:ok, distance} -> {:ok, distance + 1}
      other -> other
    end
  end

  defp error do
    {:error, "Lists must be the same length."}
  end
end
