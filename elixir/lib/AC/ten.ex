defmodule AC.Ten do
  use Problem

  defmodule Cpu do
    def parse_instruction_list(input) do
      String.split(input, "\n")
      |> Enum.filter(fn line -> String.length(line) != 0 end)
      |> Enum.map(fn line -> parse_instruction(line) end)
    end

    def run_instructions(instruction_list) do
      ins =
        Enum.flat_map_reduce(instruction_list, default_state(), fn ins, acc ->
          new_state = run_instruction(acc, ins)
          {new_state, List.last(new_state)}
        end)
        |> elem(0)

      [default_state()] ++ ins
    end

    defp parse_instruction(line) do
      arg_list = String.split(line)

      fun =
        cond do
          hd(arg_list) == "addx" -> :addx
          hd(arg_list) == "noop" -> :noop
        end

      cond do
        fun == :addx -> %{operation: fun, value: String.to_integer(hd(tl(arg_list)))}
        fun == :noop -> %{operation: fun}
      end
    end

    defp default_state() do
      %{x_register: 1, cycle_num: 1}
    end

    defp run_instruction(state, ins) do
      cond do
        ins[:operation] == :addx ->
          [
            Map.update!(state, :cycle_num, fn c -> c + 1 end),
            Map.update!(state, :x_register, fn x_reg -> x_reg + ins[:value] end)
            |> Map.update!(:cycle_num, fn c -> c + 2 end)
          ]

        ins[:operation] == :noop ->
          [Map.update!(state, :cycle_num, fn c -> c + 1 end)]
      end
    end
  end

  @spec test_input() :: String
  def test_input() do
    """
    addx 15
    addx -11
    addx 6
    addx -3
    addx 5
    addx -1
    addx -8
    addx 13
    addx 4
    noop
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx -35
    addx 1
    addx 24
    addx -19
    addx 1
    addx 16
    addx -11
    noop
    noop
    addx 21
    addx -15
    noop
    noop
    addx -3
    addx 9
    addx 1
    addx -3
    addx 8
    addx 1
    addx 5
    noop
    noop
    noop
    noop
    noop
    addx -36
    noop
    addx 1
    addx 7
    noop
    noop
    noop
    addx 2
    addx 6
    noop
    noop
    noop
    noop
    noop
    addx 1
    noop
    noop
    addx 7
    addx 1
    noop
    addx -13
    addx 13
    addx 7
    noop
    addx 1
    addx -33
    noop
    noop
    noop
    addx 2
    noop
    noop
    noop
    addx 8
    noop
    addx -1
    addx 2
    addx 1
    noop
    addx 17
    addx -9
    addx 1
    addx 1
    addx -3
    addx 11
    noop
    noop
    addx 1
    noop
    addx 1
    noop
    noop
    addx -13
    addx -19
    addx 1
    addx 3
    addx 26
    addx -30
    addx 12
    addx -1
    addx 3
    addx 1
    noop
    noop
    noop
    addx -9
    addx 18
    addx 1
    addx 2
    noop
    noop
    addx 9
    noop
    noop
    noop
    addx -1
    addx 2
    addx -37
    addx 1
    addx 3
    noop
    addx 15
    addx -21
    addx 22
    addx -6
    addx 1
    noop
    addx 2
    addx 1
    noop
    addx -10
    noop
    noop
    addx 20
    addx 1
    addx 2
    addx 2
    addx -6
    addx -11
    noop
    noop
    noop
    """
  end

  def test_output_part1 do
    13140
  end

  @spec test_output_part2 :: :better
  def test_output_part2 do
    o = """
    ##..##..##..##..##..##..##..##..##..##..
    ###...###...###...###...###...###...###.
    ####....####....####....####....####....
    #####.....#####.....#####.....#####.....
    ######......######......######......####
    #######.......#######.......#######.....
    """

    String.split_at(o, String.length(o) - 1)
    |> elem(0)
  end

  def problem1(input) do
    Cpu.parse_instruction_list(input)
    |> Cpu.run_instructions()
    |> Enum.filter(fn state -> filter_cycle(state[:cycle_num]) end)
    |> Enum.map(fn state -> state[:cycle_num] * state[:x_register] end)
    |> Enum.sum()
  end

  def filter_cycle(cycle_num) do
    if cycle_num < 40 do
      cycle_num == 20
    else
      rem(cycle_num - 20, 40) == 0
    end
  end

  def problem2(input) do
    Cpu.parse_instruction_list(input)
    |> Cpu.run_instructions()
    |> Enum.map(fn state -> get_state_char(state) end)
    |> Enum.chunk_every(40)
    |> Enum.map(fn row -> List.to_string(row) end)
    |> Enum.filter(fn s -> String.length(s) == 40 end)
    |> Enum.reduce("", fn x, acc ->
      if String.length(acc) == 0 do
        x
      else
        acc <> "\n" <> x
      end
    end)
  end

  def get_state_char(state) do
    in_sprite = run_ins_ray(state)

    cond do
      in_sprite -> "#"
      !in_sprite -> "."
    end
  end

  def run_ins_ray(ins) do
    put_pound = (rem(ins[:cycle_num] - 1, 40) - ins[:x_register]) |> abs()
    put_pound == 0 || put_pound == 1
  end
end
