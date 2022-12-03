defmodule ACTest do
  use ExUnit.Case
  doctest AC

  test "test module" do
    test_output = AC.TestProblem.test_input()
    assert AC.TestProblem.problem1() == AC.TestProblem.test_output_part1()
    assert AC.TestProblem.problem1(test_output) == AC.TestProblem.test_output_part1()

    assert AC.TestProblem.problem2() == AC.TestProblem.test_output_part2()
    assert AC.TestProblem.problem2(test_output) == AC.TestProblem.test_output_part2()
  end

  test "advent 1" do
    assert AC.One.problem1() == AC.One.test_output_part1()

    assert AC.One.problem2() == AC.One.test_output_part2()
  end

  test "advent 2" do
    assert AC.Two.problem1() == AC.Two.test_output_part1()

    assert AC.Two.problem2() == AC.Two.test_output_part2()
  end

  test "advent 3" do
    assert AC.Three.problem1() == AC.Three.test_output_part1()

    assert AC.Three.problem2() == AC.Three.test_output_part2()
  end

  test "advent 3 split line" do
    assert AC.Three.split_line("aabb") == %{left: "aa", right: "bb"}
  end

  test "advent 3 find repeat" do
    # assert AC.Three.find_repeat(%{left: "asv2",right: "s1xl"}) == "s"
    assert AC.Three.find_repeat("asv2", "s1sl") == "s"
  end

  test "advent 3 list to string" do
    # assert AC.Three.find_repeat(%{left: "asv2",right: "s1xl"}) == "s"
    assert AC.Three.list_to_string(["a", "b", "cd"]) == "abcd"
  end
end
