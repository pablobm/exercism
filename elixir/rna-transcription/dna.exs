defmodule DNA do
  @doc """
  Transcribes a character list representing DNA nucleotides to RNA

  ## Examples

  iex> DNA.to_rna('ACTG')
  'UGAC'
  """
  @spec to_rna([char]) :: [char]
  def to_rna([]) do
    []
  end

  def to_rna([h | t]) do
    [dna_to_rna(h) | to_rna(t)]
  end

  defp dna_to_rna(n) do
    %{
      ?A => ?U,
      ?T => ?A,
      ?G => ?C,
      ?C => ?G,
    }[n]
  end
end
