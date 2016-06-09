defmodule Bob do
  def hey(input) do
    cond do
      String.strip(input) == "" -> "Fine. Be that way!"
      String.last(input) == "?" -> "Sure."
      input == String.upcase(input) && input != String.downcase(input) -> "Whoa, chill out!"
      true -> "Whatever."
    end
  end
end
