defmodule AC.One do
  @moduledoc """
  Solution to AOC 1
  """
  use Problem

  def test_input() do
    """
    1000
    2000
    3000

    4000

    5000
    6000

    7000
    8000
    9000

    10000
    """
  end

  def test_output_part1 do
    24000
  end

  def test_output_part2 do
    45000
  end

  @doc """
  First advent of code, takes in path of test input
  iex> AC.one("./test/test_input/1")
  24000
  """
  def problem1(input) do
    parse_to_struct(input)
    |> Enum.reduce(0, fn x, acc ->
      cond do
        x[:sum] > acc -> x[:sum]
        true -> acc
      end
    end)
  end

  def problem2(input) do
    parse_to_struct(input)
    |> Enum.map(fn x -> x[:sum] end)
    |> Enum.sort(fn x, y -> x >= y end)
    |> Enum.take(3)
    |> Enum.sum()
  end

  def parse_to_struct(str) do
    String.split(str, "\n")
    |> Enum.chunk_by(fn x -> x == "" end)
    |> Enum.filter(fn x -> x != [""] end)
    |> Enum.map(fn arr -> AC.One.parse_int_list(arr) end)
    |> Enum.map(fn arr -> AC.One.to_sum_struct(arr) end)
  end

  def parse_int(s) do
    {num, _} = Integer.parse(s)
    num
  end

  def parse_int_list(l) do
    Enum.map(l, fn x ->
      parse_int(x)
    end)
  end

  def to_sum_struct(foods) do
    %{sum: Enum.sum(foods), elements: foods}
  end
end
