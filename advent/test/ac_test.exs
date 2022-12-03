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
  test "advent 2p1" do
    input =
"""
A Y
B X
C Z
"""
    assert AC.two_p1(input) == 15
  end
  test "advent 2p2" do
    input =
"""
A Y
B X
C Z
"""
    assert AC.two_p2(input) == 12
  end
end
