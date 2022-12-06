defmodule AC.Five do
  use Problem
  @spec test_input :: String
  def test_input() do
    """
        [D]
    [N] [C]
    [Z] [M] [P]
     1   2   3

    move 1 from 2 to 1
    move 3 from 1 to 3
    move 2 from 2 to 1
    move 1 from 1 to 2
    """
  end

  def test_output_part1 do
    "CMZ"
  end

  def test_output_part2 do
    "MCD"
  end

  @spec problem1(String) :: String
  def problem1(input) do
    handle_crane(input, true)
  end

  def problem2(input) do
    handle_crane(input, false)
  end

  defp handle_crane(input, one_by_one) do
    l = String.split(input, "\n\n")
    initial_map = parse_initial(hd(l))

    process_instructions(initial_map, parse_instructions(hd(tl(l))), one_by_one)
    |> Map.to_list()
    |> Enum.map(fn {_bucket_atom, letters} ->
      if !Enum.empty?(letters) do
        hd(letters)
      else
        ""
      end
    end)
    |> Enum.reduce("", fn x, acc -> acc <> x end)
  end

  defp process_instructions(initial_state, instructions, one_by_one) do
    Enum.reduce(instructions, initial_state, fn instruction, state_map ->
      process_single_instruction(state_map, instruction, one_by_one)
    end)
  end

  defp process_single_instruction(state_map, instruction, one_by_one) do
    {popped, remaining} = pop_items_off_list(state_map[instruction[:from]], instruction[:num])

    state_map =
      Map.update(state_map, instruction[:from], [], fn _initial_value ->
        remaining
      end)

    popped =
      if one_by_one == true do
        Enum.reverse(popped)
      else
        popped
      end

    Map.update(state_map, instruction[:to], [], fn initial_value ->
      popped ++ initial_value
    end)
  end

  def parse_instructions(instructions_str) do
    String.split(instructions_str, "\n")
    |> Enum.map(fn line -> String.split(line) end)
    |> Enum.filter(fn x -> Enum.empty?(x) == false end)
    |> Enum.map(fn line ->
      %{
        num: elem(Integer.parse(get_list_idx(line, 1)), 0),
        from: idx_to_bucket(get_list_idx(line, 3)),
        to: idx_to_bucket(get_list_idx(line, 5))
      }
    end)
  end

  @spec idx_to_bucket(String) :: atom
  defp idx_to_bucket(idx) do
    {num, _} = Integer.parse(idx)
    String.to_atom(Integer.to_string(num - 1))
  end

  defp get_list_idx(list, idx) do
    cond do
      idx > 0 -> get_list_idx(tl(list), idx - 1)
      true -> hd(list)
    end
  end

  @spec pop_items_off_list([Any], Integer) :: {[Any], [Any]}
  defp pop_items_off_list(list, num_elements) do
    pop_n_items(list, [], num_elements)
  end

  defp pop_n_items(list, existing_stack, n) do
    cond do
      n > 0 -> pop_n_items(tl(list), existing_stack ++ [hd(list)], n - 1)
      true -> {existing_stack, list}
    end
  end

  def parse_initial(input) do
    String.split(input, "\n")
    |> Enum.map(fn line -> handle_bin_line(line) end)
    |> reduce_bin()
  end

  @spec reduce_bin(any) :: any
  def reduce_bin(lines) do
    # Enum.reduce(lines, %{}, fn row_map, acc ->
    #  Map.update(acc, :foo, [0], fn exist -> exist ++ [row_map] end)
    # end)
    Enum.map(lines, fn row_map -> Map.to_list(row_map) end)
    |> Enum.reduce(%{}, fn x, acc -> update_map_line(acc, x) end)
  end

  def update_map_line(map, line_list) do
    Enum.reduce(line_list, map, fn {idx_atom, letter}, acc_map ->
      Map.update(acc_map, idx_atom, [letter], fn exist -> exist ++ [letter] end)
    end)
  end

  defp handle_bin_line(line) do
    iter =
      String.graphemes(line)
      |> Enum.with_index()

    Enum.zip(iter, tl(iter))
    |> Enum.map(fn {{first_char, _first_idx}, {second_char, second_idx}} ->
      if first_char == "[" do
        {true, second_idx, second_char}
      else
        {false, second_idx, second_char}
      end
    end)
    |> Enum.filter(fn {is_box, _, _} -> is_box end)
    |> Enum.map(fn {_is_box, idx, letter} -> {Kernel.round((idx - 1) / 4), letter} end)
    |> Enum.reduce(%{}, fn {idx, letter}, acc ->
      Map.update(acc, integer_to_atom(idx), letter, fn _exist -> letter end)
    end)
  end

  defp integer_to_atom(int) do
    String.to_atom(Integer.to_string(int))
  end
end
