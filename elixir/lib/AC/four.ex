defmodule AC.Four do
  use Problem

  def test_input() do
    """
    2-4,6-8
    2-3,4-5
    5-7,7-9
    2-8,3-7
    6-6,4-6
    2-6,4-8
    """
  end

  def test_output_part1 do
    2
  end

  def test_output_part2 do
    4
  end

  def problem1(input) do
    String.split(input, "\n")
    |> Enum.filter(fn line -> String.length(line) != 0 end)
    |> Enum.map(fn line -> String.split(line, ",") end)
    |> Enum.map(fn line -> to_range_pair(line) end)
    |> Enum.map(fn {a, b} -> range_contains_other(a, b) end)
    |> Enum.filter(fn a -> a end)
    |> Enum.count()
  end

  @doc """
  returns true if one range fully contains another
  """
  def range_contains_other(a, b) do
    if range_a_contains_b(a, b) || range_a_contains_b(b, a) do
      true
    else
      false
    end
  end

  @doc """
  returns true if range a fully contains range b
  """
  def range_a_contains_b(a, b) do
    if a[:start] <= b[:start] && a[:end] >= b[:end] do
      true
    else
      false
    end
  end

  @doc """
  Parses pair of ranges
  """
  def to_range_pair(pair) do
    {
      to_range(hd(pair)),
      to_range(hd(tl(pair)))
    }
  end

  def to_range(str) do
    range_str = to_range_str(str)
    {range_start, _} = Integer.parse(range_str[:start])
    {range_end, _} = Integer.parse(range_str[:end])
    %{start: range_start, end: range_end}
  end

  @doc """
  parses to a range string
  """
  def to_range_str(str) do
    s_list = String.split(str, "-")
    %{start: hd(s_list), end: hd(tl(s_list))}
  end

  def problem2(input) do
    String.split(input, "\n")
    |> Enum.filter(fn line -> String.length(line) != 0 end)
    |> Enum.map(fn line -> String.split(line, ",") end)
    |> Enum.map(fn line -> to_range_pair(line) end)
    |> Enum.map(fn {a, b} -> range_contains_any_overlap(a, b) end)
    |> Enum.filter(fn a -> a end)
    |> Enum.count()
  end

  def range_contains_any_overlap(a, b) do
    range_contains_num(a, b[:start]) || range_contains_num(a, b[:end]) ||
      range_contains_num(b, a[:start]) || range_contains_num(b, a[:end])
  end

  def range_contains_num(range, num) do
    range[:start] <= num && range[:end] >= num
  end
end
