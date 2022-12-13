defmodule AC.Eleven do
  use Problem
  @spec test_input() :: String
  def test_input() do
    """
    Monkey 0:
    Starting items: 79, 98
    Operation: new = old * 19
    Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

    Monkey 1:
    Starting items: 54, 65, 75, 74
    Operation: new = old + 6
    Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

    Monkey 2:
    Starting items: 79, 60, 97
    Operation: new = old * old
    Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

    Monkey 3:
    Starting items: 74
    Operation: new = old + 3
    Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
    """
  end

  def test_output_part1 do
    10605
  end

  def test_output_part2 do
    2_713_310_158
  end

  def problem1(input) do
    parse_monkey_list_from_str(input)
    |> run_n_rounds(20, fn inspect -> div(inspect, 3) end)
    |> get_monkey_num()
  end

  def problem2(input) do
    parse_monkey_list_from_str(input)
    |> run_n_rounds(10000, fn x -> x end)
    |> get_monkey_num()
  end

  defp get_monkey_num(monkeys) do
    Tuple.to_list(monkeys)
    |> Enum.map(fn monkey -> monkey[:num_inspected] end)
    |> Enum.sort()
    |> Enum.split(-2)
    |> elem(1)
    |> Enum.product()
  end

  def reduce_monkey_items(monkeys) do
    monkey_list = Tuple.to_list(monkeys)

    factor =
      Enum.map(monkey_list, fn mon -> mon[:test][:test_cond][:by] end)
      |> Enum.product()

    Enum.map(monkey_list, fn mon ->
      Map.update!(mon, :starting_items, fn item_list ->
        Enum.map(item_list, fn item -> rem(item, factor) end)
      end)
    end)
    |> List.to_tuple()
  end

  def parse_monkey_list_from_str(input) do
    String.split(input, "\n\n")
    |> Enum.map(fn mon -> parse_monkey(mon) end)
    |> List.to_tuple()
  end

  def get_counts(monkey_steps) do
    Enum.map(monkey_steps, fn step ->
      Enum.map(step, fn mon ->
        %{monkey_num: mon[:monkey_num], num_items: Enum.count(mon[:items])}
      end)
    end)
    |> Enum.zip()
    |> Enum.map(fn tu ->
      Tuple.to_list(tu) |> Enum.map(fn mon -> mon[:num_items] end) |> Enum.sum()
    end)
    |> Enum.with_index()
  end

  def parse_monkey_list(monkey_list) do
    Enum.map(monkey_list, fn mon_step ->
      Tuple.to_list(mon_step)
      |> Enum.with_index()
      |> Enum.map(fn {step, idx} -> %{monkey_num: idx, items: step[:starting_items]} end)
      |> Tuple.to_list()
    end)

    # |> Enum.with_index()
  end

  def run_n_rounds(monkeys, n, bored_fn) do
    Enum.reduce(AC.range(0, n), monkeys, fn _, monkeys ->
      monkeys = run_round(monkeys, bored_fn)
      reduce_monkey_items(monkeys)
      # monkeys
    end)
  end

  def run_round(monkeys, bored_fn) do
    Enum.reduce(AC.range(0, tuple_size(monkeys)), monkeys, fn index, monkeys ->
      run_single_monkey_step(monkeys, index, fn x -> bored_fn.(x) end)
    end)
  end

  def run_single_monkey_step(monkeys, index, bored_fn) do
    monkey_idx = elem(monkeys, index)
    num_items = Enum.count(monkey_idx[:starting_items])

    monkeys =
      put_elem(
        monkeys,
        index,
        monkey_idx |> Map.update(:num_inspected, num_items, fn x -> x + num_items end)
      )

    Enum.reduce(elem(monkeys, index)[:starting_items], monkeys, fn _x, acc ->
      run_single_monkey_single_item(acc, index, fn x -> bored_fn.(x) end)
    end)
  end

  def run_single_monkey_single_item(monkeys, index, bored_fn) do
    monkey = elem(monkeys, index)

    inspect = hd(monkey[:starting_items])

    inspect = run_operation(monkey[:operation], inspect)

    inspect = bored_fn.(inspect)
    cond_res = get_test_result(monkey[:test][:test_cond], inspect)

    {new_item_val, throw_to} =
      cond do
        cond_res == true ->
          {inspect, monkey[:test][:true_action][:to_monkey]}

        cond_res == false ->
          {inspect, monkey[:test][:false_action][:to_monkey]}
      end

    monkey = Map.update!(monkey, :starting_items, fn items -> tl(items) end)
    monkeys = put_elem(monkeys, index, monkey)
    new_monkey = elem(monkeys, throw_to)
    new_monkey = Map.update!(new_monkey, :starting_items, fn items -> items ++ [new_item_val] end)
    monkeys = put_elem(monkeys, throw_to, new_monkey)
    monkeys
  end

  # gets if cond is true or false
  defp get_test_result(test_cond, item) do
    cond do
      test_cond[:test] == :divisible -> rem(item, test_cond[:by]) == 0
    end
  end

  # runs operation on item and returns new worry level
  defp run_operation(op, item) do
    left = get_operand_type(op[:left], item)

    right = get_operand_type(op[:right], item)

    cond do
      op[:operation_type] == :multiply -> left * right
      op[:operation_type] == :add -> left + right
    end
  end

  defp get_operand_type(operand, item) do
    cond do
      operand[:type] == :old -> item
      operand[:type] == :constant -> operand[:num]
    end
  end

  defp parse_monkey(monkey_pkt) do
    {monkey, starting_items, operation, test_cond, true_act, false_act} =
      String.split(monkey_pkt, "\n")
      |> Enum.filter(fn line -> String.length(line) != 0 end)
      |> List.to_tuple()

    %{
      monkey_num: parse_monkey_num(monkey),
      starting_items: parse_starting_items(starting_items),
      operation: parse_operation(operation),
      test: parse_test(test_cond, true_act, false_act)
    }
  end

  defp parse_test(test_cond_str, true_act, false_act) do
    %{
      test_cond: parse_test_cond(test_cond_str),
      true_action: parse_action(true_act),
      false_action: parse_action(false_act)
    }
  end

  defp parse_action(action_str) do
    %{to_monkey: String.split(action_str) |> List.last() |> String.to_integer()}
  end

  defp parse_test_cond(test_cond_str) do
    {test, _, by} = String.split(test_cond_str) |> tl() |> List.to_tuple()

    test =
      cond do
        test == "divisible" -> :divisible
      end

    %{test: test, by: String.to_integer(by)}
  end

  defp parse_operation(op_str) do
    {left_str, op_type_str, right_str} =
      String.split(op_str) |> tl() |> tl() |> tl() |> List.to_tuple()

    op_type =
      cond do
        op_type_str == "*" -> :multiply
        op_type_str == "+" -> :add
      end

    %{operation_type: op_type, left: parse_num(left_str), right: parse_num(right_str)}
  end

  defp parse_num(num_str) do
    if num_str == "old" do
      %{type: :old}
    else
      %{type: :constant, num: String.to_integer(num_str)}
    end
  end

  defp parse_starting_items(str) do
    String.split(str)
    |> tl()
    |> tl()
    |> Enum.map(fn num_str ->
      if String.split_at(num_str, -1) |> elem(1) == "," do
        String.split_at(num_str, -1) |> elem(0)
      else
        num_str
      end
    end)
    |> Enum.map(fn num_str -> String.to_integer(num_str) end)
  end

  defp parse_monkey_num(str) do
    String.split(str) |> tl() |> hd() |> String.split_at(-1) |> elem(0) |> String.to_integer()
  end
end
