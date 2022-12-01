defmodule AC do
  @moduledoc """
  Documentation for `AC`.
  """

  @doc """
  Hello world.

  ## Examples

      iex> AC.hello()
      :world

  """
  def hello do
    :world
  end
  def one(path) do
    AC.OneHelper.load_file(path) |> String.split("\n")
    |> Enum.chunk_by(fn(x) -> x=="" end)
    |> Enum.filter(fn x -> x != [""] end)
    |> Enum.map(
      fn arr -> AC.OneHelper.parse_int_list(arr)
      end)
    |> Enum.map(fn arr -> AC.OneHelper.to_sum_struct(arr)end)
    |> Enum.reduce( 0,fn x,acc ->
      cond do
        x[:sum] > acc -> x[:sum]
        true -> acc
      end
    end)
  end
  def two(path) do
    AC.OneHelper.load_file(path)
      |> AC.OneHelper.parse_to_int_list()
      |> Enum.map(fn x -> x[:sum] end)
      |> Enum.sort(fn x,y -> x >= y end)
      |> Enum.take(3)
      |> Enum.sum()
  end
end
defmodule AC.OneHelper do
  def load_file(path) do
    {:ok,file_content} = File.read(path)
    file_content
  end
  def parse_to_int_list(str) do
    String.split(str, "\n")
    |> Enum.chunk_by(fn(x) -> x=="" end)
    |> Enum.filter(fn x -> x != [""] end)
    |> Enum.map(
      fn arr -> AC.OneHelper.parse_int_list(arr)
      end)
    |> Enum.map(fn arr -> AC.OneHelper.to_sum_struct(arr)end)
  end
  def parse_int(s) do
    {num,r} = Integer.parse(s)
        num
  end
  def parse_int_list(l) do
    Enum.map(l,fn x ->
      parse_int(x)
    end)
  end
  def to_sum_struct(foods) do
    %{sum: Enum.sum(foods),elements: foods}
  end
  defmodule FoodSum do
    defstruct sum: 0, elements: []
  end
end