defmodule ProblemReq do
  @doc """
  input that AOC gives you
  """
  @callback test_input() :: String
  @doc """
  input that AOC gives you for part 1
  """
  @callback test_output_part1() :: String

   @doc """
  input that AOC gives you for part 2
  """
  @callback test_output_part2() :: String
  @callback problem1(input :: String) :: any
  @callback problem2(input :: String) :: any

end
defmodule Problem do
  defmacro __using__(_opts) do
    quote do          # <--
      import Problem     # <--
      @behaviour ProblemReq
      def problem1() do
        problem1(
          test_input()
        )
      end

      def problem2() do
        problem2(
          test_input()
        )
      end

    end               # <--
  end

end
defmodule AC.TestProblem do
  use Problem
  def test_input() do
    "Hi there!"
  end
  def test_output_part1 do
    :good
  end
  def test_output_part2 do
    :better
  end
  def problem1(_input) do
    :good
  end

  def problem2(_input) do
    :better
  end
end
defmodule AC do

  @moduledoc """
  Documentation for `AC`.
  """

  def two_test_input() do
"""
A Y
B X
C Z
"""
  end

end
defmodule GeneralHelpers do
  def load_file(path) do
    {:ok,file_content} = File.read(path)
    file_content
  end
end
