defmodule AC.Nine do
  defmodule Pos do
    def zero do
      %{x: 0, y: 0}
    end

    def update_x(pos, x_fn) do
      Map.update!(pos, :x, fn x -> x_fn.(x) end)
    end

    def update_y(pos, y_fn) do
      Map.update!(pos, :y, fn y -> y_fn.(y) end)
    end

    def put_x(pos, x) do
      Map.put(pos, :x, x)
    end

    def put_y(pos, y) do
      Map.put(pos, :y, y)
    end

    @doc """
    gets distance from a to b
    """
    def dist(a, b) do
      %{x: a[:x] - b[:x], y: a[:y] - b[:y]}
    end

    def pos_max(pos) do
      max(pos[:x], pos[:y])
    end

    def pos_abs(pos) do
      %{x: abs(pos[:x]), y: abs(pos[:y])}
    end

    @doc """
    Returns tail pos after head follows tail
    """
    def head_follow_tail(head, tail) do
      distance = dist(head, tail)

      if pos_abs(distance) |> pos_max() > 1 do
        Map.update!(tail, :x, fn x ->
          cond do
            distance[:x] < 0 -> -1 + x
            distance[:x] == 0 -> x
            distance[:x] > 0 -> x + 1
          end
        end)
        |> Map.update!(:y, fn y ->
          cond do
            distance[:y] < 0 -> -1 + y
            distance[:y] == 0 -> y
            distance[:y] > 0 -> 1 + y
          end
        end)
      else
        tail
      end
    end
  end

  use Problem
  @spec test_input() :: String
  def test_input() do
    """
    R 4
    U 4
    L 3
    D 1
    R 4
    D 1
    L 5
    R 2
    """
  end

  def large_input() do
    """
    R 5
    U 8
    L 8
    D 3
    R 17
    D 10
    L 25
    U 20
    """
  end

  def test_output_part1 do
    13
  end

  def test_output_part2 do
    1
  end

  def problem1(input) do
    parse_ins(input)
    |> Enum.reduce(%{head: Pos.zero(), tail: Pos.zero(), tail_visited: []}, fn instruction, acc ->
      {new_head, new_tail} = follow_ins(instruction, acc[:head], acc[:tail])

      Map.put(acc, :tail, new_tail)
      |> Map.update!(:tail_visited, fn vis -> vis ++ [new_tail] end)
      |> Map.put(:head, new_head)
    end)
    |> Map.get(:tail_visited)
    |> Enum.uniq()
    |> Enum.count()
  end

  def problem2(input) do
    parse_ins(input)
    |> Enum.reduce(
      %{tails: make_tail_list(9), head: Pos.zero(), tail_pos_list: [Pos.zero()]},
      fn instruction, acc ->
        {new_head, new_tails} = many_tails_follow(instruction, acc[:head], acc[:tails])

        Map.put(acc, :head, new_head)
        |> Map.put(:tails, new_tails)
        |> Map.update!(:tail_pos_list, fn tl_list ->
          tl_list ++ [Enum.at(new_tails, Enum.count(new_tails) - 1)]
        end)
      end
    )
    |> Map.get(:tail_pos_list)
    |> Enum.uniq()
    |> Enum.count()
  end

  @spec apply_ins(nil | maybe_improper_list | map, map) :: map
  def apply_ins(ins, pos) do
    cond do
      ins[:direction] == :left ->
        Pos.update_x(pos, fn x -> x - 1 end)

      ins[:direction] == :right ->
        Pos.update_x(pos, fn x -> x + 1 end)

      ins[:direction] == :up ->
        Pos.update_y(pos, fn y -> y + 1 end)

      ins[:direction] == :down ->
        Pos.update_y(pos, fn y -> y - 1 end)
    end
  end

  def make_tail_list(n) do
    Enum.map(0..(n - 1), fn _ -> Pos.zero() end)
  end

  def many_tails_follow(instruction, head, tail_list) do
    new_head = apply_ins(instruction, head)

    {
      new_head,
      Enum.map_reduce(tail_list, new_head, fn tail, head ->
        new_tail = Pos.head_follow_tail(head, tail)

        {new_tail, new_tail}
      end)
      |> elem(0)
      # |> Enum.to_list()
    }
  end

  defp get_closer_recurs(head, tails) do
  end

  # returns {head_pos,tail_pos}
  def follow_ins(instruction, head, tail) do
    new_head = apply_ins(instruction, head)
    new_tail = Pos.head_follow_tail(new_head, tail)
    {new_head, new_tail}
  end

  def parse_ins(str) do
    String.split(str, "\n")
    |> Enum.filter(fn line -> String.length(line) != 0 end)
    |> Enum.map(fn line ->
      {letter, num} = String.split(line) |> List.to_tuple()
      num = String.to_integer(num)

      cond do
        letter == "R" -> %{direction: :right, count: num}
        letter == "L" -> %{direction: :left, count: num}
        letter == "U" -> %{direction: :up, count: num}
        letter == "D" -> %{direction: :down, count: num}
      end
    end)
    |> Enum.flat_map(fn order ->
      Enum.map(AC.range(0, order[:count]), fn _i -> %{direction: order[:direction]} end)
    end)
  end
end
