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
end
