defmodule ListOps do
  # Please don't use any external modules (especially List) in your
  # implementation. The point of this exercise is to create these basic functions
  # yourself.
  #
  # Note that `++` is a function from an external module (Kernel, which is
  # automatically imported) and so shouldn't be used either.

  @spec count(list) :: non_neg_integer
  def count(l)

  def count([h | t]) do
    count(t) + 1
  end

  def count([]) do
    0
  end

  @spec reverse(list) :: list
  def reverse(l, new_list \\ [])

  def reverse([], new_list) do
    new_list
  end

  def reverse([h | t], new_list) do
    reverse(t, [h | new_list])
  end

  @spec map(list, (any -> any)) :: list
  def map(l, f)

  def map([], f) do
    []
  end

  def map([h | t], f) do
    [f.(h) | map(t, f)]
  end

  @spec filter(list, (any -> as_boolean(term))) :: list
  def filter(l, f)

  def filter([], f) do
    []
  end

  def filter([h | t], f) do
    if f.(h) do
      [h | filter(t, f)]
    else
      filter(t, f)
    end
  end

  @type acc :: any
  @spec reduce(list, acc, ((any, acc) -> acc)) :: acc
  def reduce(l, acc, f)

  def reduce([], acc, f) do
    acc
  end

  def reduce([h | t], acc, f) do
    reduce(t, f.(h, acc), f)
  end

  @spec append(list, list) :: list
  def append(a, b)

  def append([], []) do
    []
  end

  def append([], l) do
    l
  end

  def append([h | t], list) do
    [h | append(t, list)]
  end

  @spec concat([[any]]) :: [any]
  def concat(ll)

  def concat([]) do
    []
  end

  def concat([h | t]) do
    append(h, concat(t))
  end
end
