defmodule AC.Six do
  use Problem
  @spec test_input() :: String
  def test_input() do
    "mjqjpqmgbljsphdztnvjfqwrcgsmlb"
  end

  def test_output_part1 do
    7
  end

  def test_output_part2 do
    19
  end

  def problem1(input) do
    problem_runner(input, 4)
  end

  def problem2(input) do
    problem_runner(input, 14)
  end

  def problem_runner(input, num_letters) do
    String.graphemes(input)
    |> trailing_iter(num_letters)
    |> Enum.zip()
    |> Enum.map(fn x -> Tuple.to_list(x) end)
    |> Enum.map(fn x -> Enum.uniq(x) end)
    |> Enum.map(fn x -> Enum.count(x) end)
    |> Enum.with_index()
    |> Enum.filter(fn {length, _index} -> length == num_letters end)
    |> Enum.map(fn {_length, index} -> index + num_letters end)
    |> Enum.take(1)
    |> hd()
  end

  def trailing_iter(list, num_chunks) do
    make_next_list(list, num_chunks, [])
  end

  def make_next_list(list, n, current_stack) do
    cond do
      n > 0 -> make_next_list(tl(list), n - 1, current_stack ++ [list])
      true -> current_stack
    end
  end
end
