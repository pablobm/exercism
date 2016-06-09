defmodule Bob do
  def hey(input) do
    cond do
      silent?(input)
        -> "Fine. Be that way!"
      question?(input)
        -> "Sure."
      yelling?(input)
        -> "Whoa, chill out!"
      true
        -> "Whatever."
    end
  end

  defp silent?(phrase) do
    String.strip(phrase) == ""
  end

  defp question?(phrase) do
    phrase |> String.ends_with?("?")
  end

  defp yelling?(phrase) do
       phrase == String.upcase(phrase)
    && phrase != String.downcase(phrase)
  end
end
