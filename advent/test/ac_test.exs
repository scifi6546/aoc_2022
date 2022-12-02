defmodule ACTest do
  use ExUnit.Case
  doctest AC
  test "advent 1" do
    assert AC.one("./test/test_input/1") == 24000
  end
  test "advent 1p2" do
    assert AC.one_p2("./test/test_input/1") == 45000
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
