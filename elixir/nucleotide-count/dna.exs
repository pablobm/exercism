defmodule DNA do
  @nucleotides [?A, ?C, ?G, ?T]

  @doc """
  Counts individual nucleotides in a DNA strand.

  ## Examples

  iex> DNA.count('AATAA', ?A)
  4

  iex> DNA.count('AATAA', ?T)
  1
  """
  @spec count([char], char) :: non_neg_integer
  def count([], n) do
    validate_nucleotide!(n)
    0
  end

  def count([h | t], h) do
    validate_nucleotide!(h)
    1 + count(t, h)
  end

  def count([h | t], n) do
    validate_nucleotide!(n)
    validate_nucleotide!(h)
    0 + count(t, n)
  end


  @doc """
  Returns a summary of counts by nucleotide.

  ## Examples

  iex> DNA.histogram('AATAA')
  %{?A => 4, ?T => 1, ?C => 0, ?G => 0}
  """
  @spec histogram([char]) :: map
  def histogram(strand) do
    %{
      ?A => count(strand, ?A),
      ?T => count(strand, ?T),
      ?C => count(strand, ?C),
      ?G => count(strand, ?G),
    }
  end

  defp validate_nucleotide!(n) do
    case n do
      ?A -> true
      ?T -> true
      ?C -> true
      ?G -> true
      _  -> raise ArgumentError
    end
  end

end
