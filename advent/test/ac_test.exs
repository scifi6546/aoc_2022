defmodule ACTest do
  use ExUnit.Case
  doctest AC
  test "advent 1" do
    assert AC.one("./test/test_input/1") == 24000
  end
  test "advent 2" do
    assert AC.one_p2("./test/test_input/1") == 45000
  end

end
